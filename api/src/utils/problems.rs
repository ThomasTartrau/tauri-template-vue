use std::{borrow::Cow, fmt::Display};

use actix_web::{error::JsonPayloadError, HttpResponse, ResponseError};
use http::StatusCode;
use http_api_problem::{HttpApiProblem, PROBLEM_JSON_MEDIA_TYPE};
use log::{error, warn};
use paperclip::actix::api_v2_errors;
use serde_json::{to_value, Value};
use sqlx::{postgres::PgDatabaseError, Error};
use strum::EnumIter;


#[api_v2_errors(code = 403, code = 500, code = 400, code = 404, code = 409)]
#[derive(Debug, Clone, EnumIter, strum::Display)]
pub enum MyProblem {
    // Functionnal errors
    PasswordTooShort(u8),
    EmailNotVerified,

    // Auth errors
    AuthFailedLogin,
    AuthFailedRefresh,
    AuthInvalidBiscuit,
    AuthBiscuitLookupError,
    AuthInvalidAuthorizationHeader,
    AuthNoAuthorizationHeader,
    AuthEmailExpired,

    // Generics errors
    Validation(validator::ValidationErrors),
    NotFound,
    InternalServerError,
    Forbidden,
}

impl From<sqlx::Error> for MyProblem {
    fn from(e: Error) -> Self {
        match e {
            Error::RowNotFound => MyProblem::NotFound,
            Error::Database(ex) => {
                // Goal map Box<dyn DatabaseError> to PgDatabaseError
                let pg_error: &PgDatabaseError = ex.try_downcast_ref::<PgDatabaseError>().unwrap();

                //let pg_error: PgDatabaseError = ex.into();

                match pg_error.constraint() {
                    _ => {
                        error!("Database error: {}", &pg_error);
                        MyProblem::InternalServerError
                    }
                }
            }
            err => {
                error!("{}", &err);
                MyProblem::InternalServerError
            }
        }
    }
}

impl From<lettre::error::Error> for MyProblem {
    fn from(err: lettre::error::Error) -> MyProblem {
        warn!("{err}");
        MyProblem::InternalServerError
    }
}

impl From<lettre::transport::smtp::Error> for MyProblem {
    fn from(err: lettre::transport::smtp::Error) -> MyProblem {
        warn!("{err}");
        MyProblem::InternalServerError
    }
}

impl From<mrml::prelude::parser::Error> for MyProblem {
    fn from(err: mrml::prelude::parser::Error) -> MyProblem {
        warn!("{err}");
        MyProblem::InternalServerError
    }
}

impl From<mrml::prelude::render::Error> for MyProblem {
    fn from(err: mrml::prelude::render::Error) -> MyProblem {
        warn!("{err}");
        MyProblem::InternalServerError
    }
}

impl From<MyProblem> for HttpApiProblem {
    fn from(my_problem: MyProblem) -> Self {
        let problem: Problem = my_problem.to_owned().into();
        HttpApiProblem::new(problem.status)
            .type_url(format!(
                "https://hook0.com/documentation/errors/{my_problem}",
            )) // rely on Display trait of MyProblem
            .value("id".to_owned(), &my_problem.to_string()) // also rely on Display trait of MyProblem
            .value("validation".to_owned(), &problem.validation)
            .title(problem.title)
            .detail(problem.detail)
    }
}

impl ResponseError for MyProblem {
    fn status_code(&self) -> StatusCode {
        let problem: Problem = self.to_owned().into();
        problem.status
    }

    fn error_response(&self) -> HttpResponse {
        let problem: HttpApiProblem = self.to_owned().into();

        let effective_status = problem
            .status
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        let actix_status = actix_web::http::StatusCode::from_u16(effective_status.as_u16())
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

        let json = problem.json_bytes();

        actix_web::HttpResponse::build(actix_status)
            .append_header((
                actix_web::http::header::CONTENT_TYPE,
                PROBLEM_JSON_MEDIA_TYPE,
            ))
            .body(json)
    }
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub id: MyProblem,
    pub title: &'static str,
    pub detail: Cow<'static, str>,
    pub validation: Option<Value>,
    pub status: StatusCode,
}

impl From<MyProblem> for Problem {
    fn from(problem: MyProblem) -> Self {
        match problem {
            // Functionnal errors
            MyProblem::PasswordTooShort(min_length) => Problem {
                id: MyProblem::PasswordTooShort(min_length),
                title: "Password is too short",
                detail: format!("Password must be at least {min_length} characters long.").into(),
                validation: None,
                status: StatusCode::UNPROCESSABLE_ENTITY,
            },
            MyProblem::EmailNotVerified => Problem {
                id: MyProblem::EmailNotVerified,
                title: "Email not verified",
                detail: "You must verify your email address before you can log in.".into(),
                validation: None,
                status: StatusCode::FORBIDDEN,
            },


            // Auth errors
            MyProblem::AuthFailedLogin => Problem {
                id: MyProblem::AuthFailedLogin,
                title: "Authentication failed",
                detail: "The provided credentials are invalid.".into(),
                validation: None,
                status: StatusCode::FORBIDDEN,
            },
            MyProblem::AuthFailedRefresh => Problem {
                id: MyProblem::AuthFailedRefresh,
                title: "Refreshing access token failed",
                detail: "The provided refresh token is probably invalid or expired.".into(),
                validation: None,
                status: StatusCode::UNAUTHORIZED,
            },
            MyProblem::AuthInvalidBiscuit => Problem {
                id: MyProblem::AuthInvalidBiscuit,
                title: "Invalid biscuit",
                detail: "The provided authentication token (Biscuit) is not valid, was not created using the current private key or is expired.".into(),
                validation: None,
                status: StatusCode::FORBIDDEN,
            },
            MyProblem::AuthBiscuitLookupError => Problem {
                id: MyProblem::AuthBiscuitLookupError,
                title: "Could not check database to verify if the provided Biscuit was revoked",
                detail: "This is likely to be caused by database unavailability.".into(),
                validation: None,
                status: StatusCode::INTERNAL_SERVER_ERROR,
            },
            MyProblem::AuthInvalidAuthorizationHeader => Problem {
                id: MyProblem::AuthInvalidAuthorizationHeader,
                title: "`Authorization` header is invalid",
                detail: "`Authorization` header value could not be decoded as a valid UTF-8 string containing `Bearer {UUID}`.".into(),
                validation: None,
                status: StatusCode::BAD_REQUEST,
            },
            MyProblem::AuthNoAuthorizationHeader => Problem {
                id: MyProblem::AuthNoAuthorizationHeader,
                title: "No `Authorization` header was found in the HTTP request",
                detail: "`Authorization` header must be provided and must contain a bearer token.".into(),
                validation: None,
                status: StatusCode::UNAUTHORIZED,
            },
            MyProblem::AuthEmailExpired => {
                Problem {
                    id: MyProblem::AuthEmailExpired,
                    title: "Could not verify your link",
                    detail: "The link you clicked might be expired. Please retry the whole process or contact support.".into(),
                    validation: None,
                    status: StatusCode::UNAUTHORIZED,
                }
            },

            

            // Generics errors
            MyProblem::Validation(e) => {
                let errors_str = e.to_string();
                Problem {
                    id: MyProblem::Validation(e.to_owned()),
                    title: "Provided input is malformed",
                    detail: errors_str.into(),
                    validation: to_value(e).ok(),
                    status: StatusCode::UNPROCESSABLE_ENTITY,
                }
            },
            MyProblem::NotFound => Problem {
                id: MyProblem::NotFound,
                title: "Resource not found",
                detail: "The requested resource could not be found.".into(),
                validation: None,
                status: StatusCode::NOT_FOUND,
            },
            MyProblem::InternalServerError => Problem {
                id: MyProblem::InternalServerError,
                title: "Internal server error",
                detail: "An unexpected error occurred.".into(),
                validation: None,
                status: StatusCode::INTERNAL_SERVER_ERROR,
            },
            MyProblem::Forbidden => Problem {
                id: MyProblem::Forbidden,
                title: "Forbidden",
                detail: "You do not have permission to access this resource.".into(),
                validation: None,
                status: StatusCode::FORBIDDEN,
            },
        }
    }
}

/// Simplified error type for the JSON body parser
#[derive(Debug, Clone)]
pub enum JsonPayloadProblem {
    Overflow { limit: usize },
    ContentType,
    Deserialize(String),
    Serialize(String),
    Payload(String),
    Other(String),
}

impl Default for JsonPayloadProblem {
    fn default() -> Self {
        Self::Other("".to_owned())
    }
}

impl Display for JsonPayloadProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow { limit } => write!(f, "Body is too big (maximum is {limit} bytes)"),
            Self::ContentType => {
                write!(f, "Content-Type header should be set to 'application/json'")
            }
            Self::Deserialize(e) => write!(f, "JSON deserialization error: {e}"),
            Self::Serialize(e) => write!(f, "JSON serialization error: {e}"),
            Self::Payload(e) => write!(f, "Payload error: {e}"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<JsonPayloadError> for JsonPayloadProblem {
    fn from(e: JsonPayloadError) -> Self {
        match e {
            JsonPayloadError::OverflowKnownLength { length: _, limit } => Self::Overflow { limit },
            JsonPayloadError::Overflow { limit } => Self::Overflow { limit },
            JsonPayloadError::ContentType => Self::ContentType,
            JsonPayloadError::Deserialize(e) => Self::Deserialize(e.to_string()),
            JsonPayloadError::Serialize(e) => Self::Serialize(e.to_string()),
            JsonPayloadError::Payload(e) => Self::Payload(e.to_string()),
            e => Self::Other(e.to_string()),
        }
    }
}