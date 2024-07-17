use std::time::{Duration, SystemTime};
use biscuit_auth::{builder::Fact, builder_ext::AuthorizerExt, error, macros::*, AuthorizerLimits, Biscuit, KeyPair, PrivateKey};
use chrono::{DateTime, Utc};
use log::{error, trace};
use paperclip::v2::schema::TypedData;
use serde::Serialize;
use strum::{AsRefStr, EnumIter, EnumString, VariantNames};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RootToken {
    pub biscuit: Biscuit,
    pub serialized_biscuit: String,
    pub revocation_id: Vec<u8>,
    pub expired_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizedToken {
    User(AuthorizedUserToken),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedUserToken {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedEmailVerificationToken {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedResetPasswordToken {
    pub user_id: Uuid,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::Display,
    EnumString,
    EnumIter,
    VariantNames,
    AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum Role {
    User,
    Administrator,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

impl TypedData for Role {
    fn data_type() -> paperclip::v2::models::DataType {
        paperclip::v2::models::DataType::String
    }

    fn format() -> Option<paperclip::v2::models::DataTypeFormat> {
        None
    }
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    AuthLogout,
    AuthChangePassword,
    UserSettingsChangeProfilePicture,
    UserSettingsChangeName,
    UserSettingsDeleteUser,
}

impl<'a> Action {
    pub fn action_name(&self) -> &'static str {
        match self {
            Action::AuthLogout => "auth:logout",
            Action::AuthChangePassword => "auth:change_password",
            Action::UserSettingsChangeProfilePicture => "users_settings:change_profile_picture",
            Action::UserSettingsChangeName => "users_settings:change_name",
            Action::UserSettingsDeleteUser => "users_settings:delete_user",
        }
    }

    fn allowed_roles(&self) -> Vec<Role> {
        let mut roles = vec![Role::User];

        let mut per_action_roles = match self {
            Self::AuthLogout => vec![],
            Self::AuthChangePassword => vec![],
            Self::UserSettingsChangeProfilePicture => vec![],
            Self::UserSettingsChangeName => vec![],
            Self::UserSettingsDeleteUser => vec![],
        };

        roles.append(&mut per_action_roles);
        roles
    }

    pub fn generate_facts(self) -> Vec<Fact> {
        let mut facts = match self {
            Self::AuthLogout => vec![],
            Self::AuthChangePassword => vec![],
            Self::UserSettingsChangeProfilePicture => vec![],
            Self::UserSettingsChangeName => vec![],
            Self::UserSettingsDeleteUser => vec![],
        };

        facts.push(fact!("action({action})", action = self.action_name()));

        for role in self.allowed_roles() {
            facts.push(fact!("allowed_role({role})", role = role.as_ref()));
        }

        facts
    }
}

const USER_ACCESS_TOKEN_VERSION: i64 = 1;
const USER_ACCESS_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 5); // 5 minutes

pub fn create_user_access_token(
    private_key: &PrivateKey,
    token_id: Uuid,
    session_id: Uuid,
    user_id: Uuid,
    email: &str,
    first_name: &str,
    last_name: &str,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();
    let expired_at = created_at + USER_ACCESS_TOKEN_EXPIRATION;

    let biscuit = {
        let biscuit = biscuit!(
            r#"
                type("user_access");
                version({USER_ACCESS_TOKEN_VERSION});
                session_id({session_id});
                token_id({token_id});
                created_at({created_at});
                user_id({user_id});
                email({email});
                first_name({first_name});
                last_name({last_name});

                check if time($t), $t < {expired_at};
            "#,
        );
        biscuit.build(&keypair)?
    };
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: Some(DateTime::from(expired_at)),
    })
}

const REFRESH_TOKEN_VERSION: i64 = 1;
const REFRESH_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 30);

pub fn create_refresh_token(
    private_key: &PrivateKey,
    token_id: Uuid,
    session_id: Uuid,
    user_id: Uuid,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();
    let expired_at = created_at + REFRESH_TOKEN_EXPIRATION;

    let biscuit = biscuit!(
        r#"
            type("refresh");
            version({REFRESH_TOKEN_VERSION});
            token_id({token_id});
            session_id({session_id});
            user_id({user_id});
            created_at({created_at});

            check if time($t), $t < {expired_at};
        "#,
    )
    .build(&keypair)?;
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: Some(DateTime::from(expired_at)),
    })
}

const EMAIL_VERIFICATION_TOKEN_VERSION: i64 = 1;
const EMAIL_VERIFICATION_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 30);

pub fn create_email_verification_token(
    private_key: &PrivateKey,
    user_id: Uuid,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();

    let expired_at = created_at + EMAIL_VERIFICATION_TOKEN_EXPIRATION;

    let biscuit = biscuit!(
        r#"
            type("email_verification");
            version({EMAIL_VERIFICATION_TOKEN_VERSION});
            user_id({user_id});
            created_at({created_at});
            expired_at({expired_at});
        "#,
    )
    .build(&keypair)?;
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: Some(DateTime::from(expired_at)),
    })
}

const RESET_PASSWORD_TOKEN_VERSION: i64 = 1;
const RESET_PASSWORD_TOKEN_EXPIRATION: Duration = Duration::from_secs(60 * 30);

pub fn create_reset_password_token(
    private_key: &PrivateKey,
    user_id: Uuid,
) -> Result<RootToken, biscuit_auth::error::Token> {
    let keypair = KeyPair::from(private_key);
    let created_at = SystemTime::now();
    let expired_at = created_at + RESET_PASSWORD_TOKEN_EXPIRATION;

    let biscuit = biscuit!(
        r#"
            type("password_reset");
            version({RESET_PASSWORD_TOKEN_VERSION});
            user_id({user_id});
            created_at({created_at});
            expired_at({expired_at});
        "#,
    )
    .build(&keypair)?;
    let serialized_biscuit = biscuit.to_base64()?;
    let revocation_id = biscuit
        .revocation_identifiers()
        .first()
        .map(|rid| rid.to_owned())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(RootToken {
        biscuit,
        serialized_biscuit,
        revocation_id,
        expired_at: Some(DateTime::from(expired_at)),
    })
}

pub fn authorize_only_user(
    biscuit: &Biscuit,
    action: Action,
) -> Result<AuthorizedUserToken, biscuit_auth::error::Token> {
    match authorize(biscuit, action) {
        Ok(AuthorizedToken::User(aut)) => Ok(aut),
        #[allow(unreachable_patterns)] // This is unreachable because the only authorized token is a user token
        Ok(_) => {
            trace!("Authorization was denied because a user_access token was required");
            Err(biscuit_auth::error::Token::InternalError)
        }
        Err(e) => Err(e),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedRefreshToken {
    pub token_id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
}

pub fn authorize_refresh_token(
    biscuit: &Biscuit,
) -> Result<AuthorizedRefreshToken, biscuit_auth::error::Token> {
    let mut authorizer = authorizer!(
        r#"
            supported_version("refresh", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);

            expired($t) <- expired_at($exp), time($t), $exp < $t;
            deny if expired($t);
        "#
    );
    authorizer.set_time();
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_millis(5),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_token_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- token_id($id)"))?;
    let token_id = raw_token_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;
    let raw_session_id: Vec<(Vec<u8>,)> =
        authorizer.query(rule!("data($id) <- session_id($id)"))?;
    let session_id = raw_session_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;
    let raw_user_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- user_id($id)"))?;
    let user_id = raw_user_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(AuthorizedRefreshToken {
        token_id,
        session_id,
        user_id,
    })
}

pub fn authorize(
    biscuit: &Biscuit,
    action: Action,
) -> Result<AuthorizedToken, error::Token> {

    let mut authorizer = authorizer!(
        r#"
            valid_types(["user_access"]);
            valid_type($t) <- type($t), valid_types($vt), $vt.contains($t);
            check if valid_type($t);

            supported_version("user_access", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);

            expired($t) <- expired_at($exp), time($t), $exp < $t;
            deny if expired($t);
        "#
    );

    authorizer.set_time();
    for fact in action.generate_facts() {
        authorizer.add_fact(fact)?;
    }
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_millis(5),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_type: Vec<(String,)> = authorizer.query(rule!("data($id) <- type($id)"))?;
    let token_type = raw_type
        .first()
        .map(|(str,)| str)
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    match token_type.as_str() {
        "user_access" => {
            let raw_session_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- session_id($id)"))?;
            let session_id = raw_session_id
                .first()
                .and_then(|(str,)| Uuid::from_slice(str).ok())
                .ok_or(biscuit_auth::error::Token::InternalError)?;

            let raw_user_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- user_id($id)"))?;
            let user_id = raw_user_id
                .first()
                .and_then(|(str,)| Uuid::from_slice(str).ok())
                .ok_or(biscuit_auth::error::Token::InternalError)?;

            let raw_email: Vec<(String,)> = authorizer.query(rule!("data($email) <- email($email)"))?;
            let email = raw_email
                .first()
                .ok_or(biscuit_auth::error::Token::InternalError)?
                .0
                .to_owned();

            let raw_first_name: Vec<(String,)> = authorizer.query(rule!("data($first_name) <- first_name($first_name)"))?;
            let first_name = raw_first_name
                .first()
                .ok_or(biscuit_auth::error::Token::InternalError)?
                .0
                .to_owned();

            let raw_last_name: Vec<(String,)> = authorizer.query(rule!("data($last_name) <- last_name($last_name)"))?;
            let last_name = raw_last_name
                .first()
                .ok_or(biscuit_auth::error::Token::InternalError)?
                .0
                .to_owned();

            Ok(AuthorizedToken::User(AuthorizedUserToken {
                session_id,
                user_id,
                email,
                first_name,
                last_name,
            }))
        },
        _ => {
            error!("Invalid token type: {}", token_type);
            Err(biscuit_auth::error::Token::InternalError)
        }
    }
}

pub fn authorize_email_verification(
    biscuit: &Biscuit,
) -> Result<AuthorizedEmailVerificationToken, biscuit_auth::error::Token> {
    let mut authorizer = authorizer!(
        r#"
            supported_version("email_verification", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);

            expired($t) <- expired_at($exp), time($t), $exp < $t;
            deny if expired($t);
        "#
    );
    authorizer.set_time();
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_secs(1800),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_user_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- user_id($id)"))?;
    let user_id = raw_user_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(AuthorizedEmailVerificationToken { user_id })
}

pub fn get_user_id_from_expired_email_verification(
    biscuit: &Biscuit,
) -> Result<Uuid, biscuit_auth::error::Token> {
    let mut authorizer = authorizer!(
        r#"
            supported_version("email_verification", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);
        "#
    );
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_secs(1800),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_user_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- user_id($id)"))?;
    let user_id = raw_user_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(user_id)
}

pub fn authorize_reset_password(
    biscuit: &Biscuit,
) -> Result<AuthorizedResetPasswordToken, biscuit_auth::error::Token> {
    let mut authorizer = authorizer!(
        r#"
            supported_version("password_reset", 1);
            valid_version($t, $v) <- type($t), version($v), supported_version($t, $v);
            check if valid_version($t, $v);

            expired($t) <- expired_at($exp), time($t), $exp < $t;
            deny if expired($t);
        "#
    );
    authorizer.set_time();
    authorizer.add_allow_all();

    authorizer.set_limits(AuthorizerLimits {
        max_time: Duration::from_secs(1800),
        ..Default::default()
    });
    authorizer.add_token(biscuit)?;
    let result = authorizer.authorize();
    trace!("Authorizer state:\n{}", authorizer.print_world());
    result?;

    let raw_user_id: Vec<(Vec<u8>,)> = authorizer.query(rule!("data($id) <- user_id($id)"))?;
    let user_id = raw_user_id
        .first()
        .and_then(|(str,)| Uuid::from_slice(str).ok())
        .ok_or(biscuit_auth::error::Token::InternalError)?;

    Ok(AuthorizedResetPasswordToken { user_id })
}