use actix_web::web::{Json, ReqData};
use actix_multipart::Multipart;
use biscuit_auth::Biscuit;
use futures_util::TryStreamExt;
use log::{error, warn};
use mime::{Mime, IMAGE_JPEG, IMAGE_PNG};
use paperclip::actix::web::Data;
use paperclip::actix::{api_v2_operation, Apiv2Schema, NoContent};
use serde::{Deserialize, Serialize};
use sqlx::query;
use validator::Validate;
use std::fs;
use std::io::Write;

use crate::utils::problems::MyProblem;
use crate::auth::iam::{authorize_only_user, Action};
use crate::utils::openapi::OaBiscuitUserAccess;

#[derive(Debug, Serialize, Deserialize, Apiv2Schema, Validate)]
pub struct ChangeNamePost {
    #[validate(non_control_character, length(min = 1, max = 20))]
    first_name: String,
    #[validate(non_control_character, length(min = 1, max = 20))]
    last_name: String,
}

const MAX_FILE_COUNT: usize = 1;
const MAX_FILE_SIZE: usize = 1024 * 1024 * 5; // Todo: Add file limit
const IMAGE_SIZE: (u32, u32) = (200, 200); // Todo: Add resize image
const ALLOWED_FILE_TYPES: [Mime; 2] = [IMAGE_JPEG, IMAGE_PNG];
/* #[api_v2_operation(
    summary = "Change profile picture",
    description = "Change the profile picture of a user.",
    operation_id = "user_settings.change_profile_picture",
    consumes = "multipart/form-data",
    produces = "application/json",
    tags("UserSettings")
)] */
pub async fn change_profile_picture(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    mut payload: Multipart,
) -> Result<NoContent, MyProblem> {
    if let Ok(token) = authorize_only_user(
        &biscuit,
        Action::UserSettingsChangeProfilePicture,
    ) {
        let mut current_count = 0;
        loop {
            if current_count == MAX_FILE_COUNT { break; }
            if let Some(mut field) = payload.try_next().await.map_err(|e| {
                error!("Error trying to read profile picture: {e}");
                MyProblem::InternalServerError
            })? {
                let filetype: Option<&Mime> = field.content_type();
                if filetype.is_none() { continue; }
                if !ALLOWED_FILE_TYPES.contains(&filetype.unwrap()) { continue; }

                let destination: String = format!(
                    "{}{}.jpeg",
                    state.profile_picture_dir,
                    token.user_id
                );
    
                let mut saved_file: fs::File = fs::File::create(&destination).map_err(|e| {
                    error!("Error trying to create profile picture: {e}");
                    MyProblem::InternalServerError
                })?;
                while let Ok(Some(chunk)) = field.try_next().await {
                    let _ = saved_file.write_all(&chunk).map_err(|e| {
                        error!("Error trying to write profile picture: {e}");
                        MyProblem::InternalServerError
                    })?;
                }
                
            } else { 
                warn!("No profile picture found in the request");
                break;
             }
            current_count += 1;
        }

        Ok(NoContent)
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Change name",
    description = "Change the name of the user.",
    operation_id = "user_settings.change_name",
    produces = "application/json",
    tags("UserSettings")
)]
pub async fn change_name(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
    body: Json<ChangeNamePost>,
) -> Result<NoContent, MyProblem> {
    if let Ok(token) = authorize_only_user(
        &biscuit,
        Action::UserSettingsChangeName,
    ) {
        query!(
            "UPDATE iam.user SET first_name = $1, last_name = $2 WHERE user__id = $3",
            body.first_name,
            body.last_name,
            token.user_id
        )
        .execute(&state.db)
        .await?;

        Ok(NoContent)
    } else {
        Err(MyProblem::Forbidden)
    }
}

#[api_v2_operation(
    summary = "Delete user",
    description = "Delete the user and all he's information.",
    operation_id = "user_settings.delete_user",
    produces = "application/json",
    tags("UserSettings")
)]
pub async fn delete_user(
    state: Data<crate::State>,
    _: OaBiscuitUserAccess,
    biscuit: ReqData<Biscuit>,
) -> Result<NoContent, MyProblem> {
    if let Ok(token) = authorize_only_user(
        &biscuit,
        Action::UserSettingsDeleteUser,
    ) {
        query!(
            "DELETE FROM iam.user WHERE user__id = $1",
            token.user_id
        )
        .execute(&state.db)
        .await?;

        Ok(NoContent)
    } else {
        Err(MyProblem::Forbidden)
    }
}