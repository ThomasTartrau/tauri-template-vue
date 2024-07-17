use actix_web::web::ReqData;
use argon2::password_hash::PasswordHashString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use biscuit_auth::{Biscuit, PrivateKey};
use chrono::{DateTime, Utc};
use lettre::message::Mailbox;
use lettre::Address;
use log::{debug, error, info, warn};
use paperclip::actix::web::{Data, Json};
use paperclip::actix::{api_v2_operation, Apiv2Schema, CreatedJson, NoContent};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, query_scalar, Acquire, Postgres};
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

use crate::utils::mailer::Mail;
use crate::utils::problems::MyProblem;
use crate::auth::iam::{
    authorize_email_verification, authorize_only_user, authorize_refresh_token, create_refresh_token, create_reset_password_token, create_user_access_token, authorize_reset_password, Action
};
use crate::utils::openapi::{OaBiscuitRefresh, OaBiscuitUserAccess};

use super::iam::{create_email_verification_token, get_user_id_from_expired_email_verification};

#[derive(Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct LoginPost {
    #[validate(non_control_character, length(min = 1, max = 100))]
    email: String,
    #[validate(non_control_character, length(min = 1, max = 100))]
    password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserLookup {
    user_id: Uuid,
    password_hash: String,
    email: String,
    first_name: String,
    last_name: String,
    email_verified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct LoginResponse {
    access_token: String,
    access_token_expiration: DateTime<Utc>,
    refresh_token: String,
    refresh_token_expiration: DateTime<Utc>,
    user_id: Uuid,
    email: String,
    first_name: String,
    last_name: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct EmailVerificationPost {
    #[validate(non_control_character, length(min = 1, max = 1000))]
    token: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct ResendEmailVerificationPost {
    #[validate(non_control_character, length(min = 1, max = 1000))]
    token: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct BeginResetPasswordPost {
    #[validate(non_control_character, email, length(max = 100))]
    email: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct ResetPasswordPost {
    #[validate(non_control_character, length(min = 1, max = 1000))]
    token: String,
    #[validate(non_control_character, length(min = 10, max = 100))]
    new_password: String,
}

#[derive(Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct ChangePasswordPost {
    #[validate(non_control_character, length(min = 10, max = 100))]
    new_password: String,
}

#[api_v2_operation(
    summary = "Login",
    description = "Get an access token using a user's credentials.",
    operation_id = "auth.login",
    consumes = "application/json",
    produces = "application/json",
    tags("Authentication")
)]
pub async fn login(
    state: Data<crate::State>,
    body: Json<LoginPost>,
) -> Result<CreatedJson<LoginResponse>, MyProblem> {
    if let Err(e) = body.validate() {
        return Err(MyProblem::Validation(e));
    }

    let user_lookup = query_as!(
        UserLookup,
        "
            SELECT user__id AS user_id, password AS password_hash, email, first_name, last_name, email_verified_at
            FROM iam.user
            WHERE email = $1
        ",
        &body.email,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(MyProblem::from)?;

    if let Some(user) = user_lookup {
        if user.email_verified_at.is_some() {
            let password_hash = PasswordHash::new(&user.password_hash).map_err(|e| {
                error!(
                    "Password hash of user {} is not in the right format: {e}",
                    &user.user_id
                );
                MyProblem::InternalServerError
            })?;

            if Argon2::default()
                .verify_password(body.password.as_bytes(), &password_hash)
                .is_ok()
            {
                do_login(&state.db, &state.biscuit_private_key, user, None).await
            } else {
                Err(MyProblem::AuthFailedLogin)
            }
        } else {
            Err(MyProblem::EmailNotVerified)
        }
    } else {
        Err(MyProblem::AuthFailedLogin)
    }
}

async fn do_login<'a, A: Acquire<'a, Database = Postgres>>(
    db: A,
    biscuit_private_key: &PrivateKey,
    user: UserLookup,
    session_id: Option<Uuid>,
) -> Result<CreatedJson<LoginResponse>, MyProblem> {
    let mut db = db.acquire().await?;

    let session_id = session_id.unwrap_or_else(Uuid::new_v4);
    let access_token_id = Uuid::new_v4();
    let (access_token, access_token_expiration) = create_user_access_token(
        biscuit_private_key,
        access_token_id,
        session_id,
        user.user_id,
        &user.email,
        &user.first_name,
        &user.last_name,
    )
    .and_then(|rt| {
        if let Some(expired_at) = rt.expired_at {
            Ok((rt, expired_at))
        } else {
            Err(biscuit_auth::error::Token::InternalError)
        }
    })
    .map_err(|e| {
        error!("Could not create a Biscuit (user access token): {e}");
        MyProblem::InternalServerError
    })?;

    let refresh_token_id = Uuid::new_v4();
    let (refresh_token, refresh_token_expiration) = create_refresh_token(
        biscuit_private_key,
        refresh_token_id,
        session_id,
        user.user_id,
    )
    .and_then(|rt| {
        if let Some(expired_at) = rt.expired_at {
            Ok((rt, expired_at))
        } else {
            Err(biscuit_auth::error::Token::InternalError)
        }
    })
    .map_err(|e| {
        error!("Could not create a Biscuit (refresh token): {e}");
        MyProblem::InternalServerError
    })?;

    query!(
        "
            INSERT INTO iam.token (token__id, type, revocation_id, expired_at, user__id, session_id)
            VALUES
                ($1, 'user_access', $2, $3, $4, $5),
                ($6, 'refresh', $7, $8, $4, $5)
        ",
        &access_token_id,
        &access_token.revocation_id,
        access_token_expiration,
        &user.user_id,
        &session_id,
        &refresh_token_id,
        &refresh_token.revocation_id,
        refresh_token_expiration,
    )
    .execute(&mut *db)
    .await?;

    query!(
        "
            UPDATE iam.user
            SET last_login = statement_timestamp()
            WHERE user__id = $1
        ",
        &user.user_id,
    )
    .execute(&mut *db)
    .await?;

    Ok(CreatedJson(LoginResponse {
        access_token: access_token.serialized_biscuit,
        access_token_expiration,
        refresh_token: refresh_token.serialized_biscuit,
        refresh_token_expiration,
        user_id: user.user_id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
    }))
}

#[api_v2_operation(
    summary = "Refresh access token",
    description = "Get a new access token in exchange of a refresh token.",
    operation_id = "auth.refresh",
    consumes = "application/json",
    produces = "application/json",
    tags("Authentication")
)]
pub async fn refresh(
    state: Data<crate::State>,
    _: OaBiscuitRefresh,
    biscuit: ReqData<Biscuit>,
) -> Result<CreatedJson<LoginResponse>, MyProblem> {
    if let Ok(token) = authorize_refresh_token(&biscuit) {
        let mut tx = state.db.begin().await?;

        query!(
            "
                UPDATE iam.token
                SET expired_at = statement_timestamp()
                WHERE token__id = $1
                    AND type = 'refresh'
                    AND expired_at > statement_timestamp()
            ",
            &token.token_id,
        )
        .execute(&mut *tx)
        .await?;

        let user = query_as!(
            UserLookup,
            "
                SELECT user__id AS user_id, password AS password_hash, email, first_name, last_name, email_verified_at
                FROM iam.user
                WHERE user__id = $1
            ",
            &token.user_id,
        )
        .fetch_one(&state.db)
        .await
        .map_err(MyProblem::from)?;

        let res = do_login(
            &mut tx,
            &state.biscuit_private_key,
            user,
            Some(token.session_id),
        )
        .await?;
        tx.commit().await?;
        Ok(res)
    } else {
        Err(MyProblem::AuthFailedRefresh)
    }
}

#[api_v2_operation(
    summary = "Logout",
    description = "Revoke all tokens associated to the current session.",
    operation_id = "auth.logout",
    consumes = "application/json",
    produces = "application/json",
    tags("Authentication")
)]
pub async fn logout(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<NoContent, MyProblem> {
    if let Ok(token) = authorize_only_user(&biscuit, Action::AuthLogout) {
        query!(
            "
                UPDATE iam.token
                SET expired_at = statement_timestamp()
                WHERE user__id = $1
                    AND expired_at > statement_timestamp()
                    AND session_id = $2
                    AND type IN ('user_access', 'refresh')
            ",
            &token.user_id,
            &token.session_id,
        )
        .execute(&state.db)
        .await?;

        Ok(NoContent)
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Email verification",
    description = "Verify the email of a user.",
    operation_id = "auth.verify_email",
    consumes = "application/json",
    produces = "application/json",
    tags("Authentication")
)]
pub async fn verify_email(
    state: Data<crate::State>,
    body: Json<EmailVerificationPost>,
) -> Result<NoContent, MyProblem> {
    if let Err(e) = body.validate() {
        return Err(MyProblem::Validation(e));
    }

    let body = body.into_inner();

    let token =
        Biscuit::from_base64(body.token, state.biscuit_private_key.public()).map_err(|e| {
            debug!("{e}");
            MyProblem::AuthEmailExpired
        })?;

    if let Ok(token) = authorize_email_verification(&token) {
        let user_was_verified = query!(
            "
                UPDATE iam.user
                SET email_verified_at = statement_timestamp()
                WHERE user__id = $1 AND email_verified_at IS NULL
                RETURNING user__id
            ",
            &token.user_id,
        )
        .fetch_optional(&state.db)
        .await?
        .is_some();

        if user_was_verified {
            Ok(NoContent)
        } else {
            debug!(
                "User {} tried to verify its email whereas it is already verified",
                &token.user_id
            );
            Err(MyProblem::AuthEmailExpired)
        }
    } else {
        Err(MyProblem::AuthEmailExpired)
    }
}

#[api_v2_operation(
    summary = "Resend email verification",
    description = "Resend an email with a link to verify the email of a user.",
    operation_id = "auth.resend_email_verification",
    consumes = "application/json",
    produces = "application/json",
    tags("Authentication")
)]
pub async fn resend_email_verification(
    state: Data<crate::State>,
    body: Json<ResendEmailVerificationPost>,
) -> Result<NoContent, MyProblem> {
    if let Err(e) = body.validate() {
        return Err(MyProblem::Validation(e));
    }

    let body = body.into_inner();

    let token =
        Biscuit::from_base64(body.token, state.biscuit_private_key.public()).map_err(|e| {
            debug!("{e}");
            MyProblem::AuthEmailExpired
        })?;

    if let Ok(user_id) = get_user_id_from_expired_email_verification(&token) {
        let user_lookup = query_as!(
            UserLookup,
            "
                SELECT user__id AS user_id, email, first_name, last_name, email_verified_at, password AS password_hash
                FROM iam.user
                WHERE user__id = $1 AND email_verified_at IS NULL
            ",
            &user_id,
        )
        .fetch_optional(&state.db)
        .await
        .map_err(MyProblem::from)?;
        info!("User lookup: {:?}", user_lookup);

        if let Some(user) = user_lookup {
            let verification_token =
            create_email_verification_token(&state.biscuit_private_key, user_id).map_err(|e| {
                error!("Error trying to create email verification token: {e}");
                MyProblem::InternalServerError
            })?;

            let address = Address::from_str(&user.email).map_err(|e| {
                error!("Error trying to parse email address: {e}");
                MyProblem::InternalServerError
            })?;
            let recipient = Mailbox::new(
                Some(format!("{} {}", &user.first_name, &user.last_name)),
                address,
            );
            state
                .mailer
                .send_mail(
                    Mail::VerifyUserEmail {
                        url: format!(
                            "{}verify-email?token={}",
                            state.app_url, &verification_token.serialized_biscuit
                        ),
                    },
                    recipient,
                )
                .await
                .map_err(|e| {
                    warn!("Could not send verification email: {e}");
                    e
                })?;
    
    
            Ok(NoContent)
        } else {
            Err(MyProblem::AuthEmailExpired)
        }
    } else {
        Err(MyProblem::Forbidden)
    
    }
    
}

#[api_v2_operation(
    summary = "Begin reset password",
    description = "Send an email with a link to reset the password of a user.",
    operation_id = "auth.begin_reset_password",
    consumes = "application/json",
    produces = "application/json",
    tags("Authentication")
)]
pub async fn begin_reset_password(
    state: Data<crate::State>,
    body: Json<BeginResetPasswordPost>,
) -> Result<NoContent, MyProblem> {
    if let Err(e) = body.validate() {
        return Err(MyProblem::Validation(e));
    }

    let body = body.into_inner();

    struct UserLookup {
        user_id: Uuid,
        email: String,
        first_name: String,
        last_name: String,
    }
    let user_lookup = query_as!(
        UserLookup,
        "
            SELECT user__id AS user_id, email, first_name, last_name
            FROM iam.user
            WHERE email = $1
        ",
        &body.email,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(MyProblem::from)?;

    if let Some(user) = user_lookup {
        let biscuit_token = create_reset_password_token(&state.biscuit_private_key, user.user_id)
            .map_err(|e| {
            error!("Error trying to create reset password token: {e}");
            MyProblem::InternalServerError
        })?;

        let address = Address::from_str(&user.email).map_err(|e| {
            error!("Error trying to parse email address: {e}");
            MyProblem::InternalServerError
        })?;
        let recipient = Mailbox::new(
            Some(format!("{} {}", user.first_name, user.last_name)),
            address,
        );

        match state
            .mailer
            .send_mail(
                Mail::ResetPassword {
                    url: format!(
                        "{}reset-password?token={}",
                        state.app_url, &biscuit_token.serialized_biscuit
                    ),
                },
                recipient,
            )
            .await
        {
            Ok(_) => Ok(NoContent),
            Err(e) => {
                error!("Error trying to send email: {e}");
                Err(MyProblem::InternalServerError)
            }
        }
    } else {
        Err(MyProblem::AuthEmailExpired)
    }
}

#[api_v2_operation(
    summary = "Reset password",
    description = "Reset the password of a user.",
    operation_id = "auth.reset_password",
    consumes = "application/json",
    produces = "application/json",
    tags("Authentication")
)]
pub async fn reset_password(
    state: Data<crate::State>,
    body: Json<ResetPasswordPost>,
) -> Result<NoContent, MyProblem> {
    if let Err(e) = body.validate() {
        return Err(MyProblem::Validation(e));
    }

    let body = body.into_inner();

    let token =
        Biscuit::from_base64(body.token, state.biscuit_private_key.public()).map_err(|e| {
            debug!("{e}");
            MyProblem::AuthEmailExpired
        })?;

    if let Ok(token) = authorize_reset_password(&token) {
        let uid = query_scalar!(
            "
                SELECT user__id
                FROM iam.user
                WHERE user__id = $1
            ",
            &token.user_id,
        )
        .fetch_optional(&state.db)
        .await
        .map_err(MyProblem::from)?;

        if let Some(user_id) = uid {
            let mut tx = state.db.begin().await?;

            do_change_password(
                &mut tx,
                state.password_minimum_length,
                &body.new_password,
                user_id,
            )
            .await?;

            query!(
                "
                    UPDATE iam.user
                    SET email_verified_at = statement_timestamp()
                    WHERE user__id = $1
                        AND email_verified_at IS NULL
                ",
                &user_id,
            )
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
            Ok(NoContent)
        } else {
            Err(MyProblem::AuthEmailExpired)
        }
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Change password",
    description = "Change the password of a user.",
    operation_id = "auth.change_password",
    consumes = "application/json",
    produces = "application/json",
    tags("Authentication")
)]
pub async fn change_password(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<ChangePasswordPost>,
) -> Result<NoContent, MyProblem> {
    if let Err(e) = body.validate() {
        return Err(MyProblem::Validation(e));
    }

    let body = body.into_inner();

    if let Ok(token) = authorize_only_user(
        &biscuit,
        Action::AuthChangePassword,
    ) {
        do_change_password(
            &state.db,
            state.password_minimum_length,
            &body.new_password,
            token.user_id,
        )
        .await?;

        Ok(NoContent)
    } else {
        Err(MyProblem::Forbidden)
    }
}

async fn do_change_password<'a, A: Acquire<'a, Database = Postgres>>(
    db: A,
    password_minimum_length: u8,
    new_password: &str,
    user_id: Uuid,
) -> Result<(), MyProblem> {
    if new_password.len() >= usize::from(password_minimum_length) {
        let password_hash = generate_hashed_password(new_password).map_err(|e| {
            error!("Error trying to hash user password: {e}");
            MyProblem::InternalServerError
        })?;

        let mut db = db.acquire().await?;

        query!(
            "
                UPDATE iam.user
                SET password = $1
                WHERE user__id = $2
            ",
            password_hash.as_str(),
            &user_id,
        )
        .execute(&mut *db)
        .await?;

        Ok(())
    } else {
        Err(MyProblem::PasswordTooShort(password_minimum_length))
    }
}

fn generate_hashed_password(password: &str) -> Result<PasswordHashString, MyProblem> {
    let salt =
        argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            error!("Error trying to hash user password: {e}");
            MyProblem::InternalServerError
        })?
        .serialize();

    Ok(password_hash)
}