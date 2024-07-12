use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use anyhow::anyhow;
use biscuit_auth::{Biscuit, PrivateKey};
use futures_util::future::{ok, ready, Ready};
use futures_util::Future;
use log::{debug, error, trace};
use sqlx::{query_scalar, PgPool};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use crate::utils::problems::MyProblem;

#[derive(Debug, Clone)]
pub struct BiscuitAuth {
    pub db: PgPool,
    pub biscuit_private_key: PrivateKey,
}

impl<S> Transform<S, ServiceRequest> for BiscuitAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = BiscuitAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        trace!("Initialize BiscuitAuthMiddleware");
        ok(BiscuitAuthMiddleware {
            service: Rc::new(service),
            db: self.db.clone(),
            biscuit_private_key: self.biscuit_private_key.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BiscuitAuthMiddleware<S> {
    service: Rc<S>,
    db: PgPool,
    biscuit_private_key: PrivateKey,
}

impl<S> Service<ServiceRequest> for BiscuitAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        debug!("Attempting auth using Biscuit");

        let auth_header = req.headers().get("Authorization");
        match auth_header {
            Some(auth_header_value) => {
                let auth_header_str = auth_header_value
                    .to_str()
                    .map_err(|e| anyhow!(e))
                    .map(|str| str.trim_start_matches("Bearer "));

                match auth_header_str {
                    Ok(token) => {
                        debug!("Token was extracted from request headers");

                        match Biscuit::from_base64(token, self.biscuit_private_key.public())
                            .and_then(|biscuit| {
                                biscuit
                                    .revocation_identifiers()
                                    .first()
                                    .map(|rid| (biscuit, rid.to_owned()))
                                    .ok_or(biscuit_auth::error::Token::InternalError)
                            }) {
                            Ok((biscuit, revocation_id)) => {
                                let pool = Box::new(self.db.clone());
                                let pool: &'static PgPool = Box::leak(pool);
                                let srv = Rc::clone(&self.service);
                                Box::pin(async move {
                                    let biscuit_token_id = query_scalar!(
                                        "
                                            SELECT token__id AS token_id
                                            FROM iam.token
                                            WHERE revocation_id = $1
                                                AND (expired_at IS NULL OR expired_at > statement_timestamp())
                                            LIMIT 1
                                        ",
                                        &revocation_id
                                    )
                                    .fetch_optional(pool)
                                    .await;

                                    match biscuit_token_id {
                                        Ok(Some(token_id)) => {
                                            {
                                                debug!(
                                                    "Auth with Biscuit succeeded (token ID = {})",
                                                    token_id
                                                );
                                                let mut extensions = req.extensions_mut();
                                                extensions.insert(biscuit);
                                            }
                                            srv.call(req).await
                                        }
                                        Ok(None) => {
                                            let e = MyProblem::AuthInvalidBiscuit;
                                            debug!("{e} (root token was not found in database or was expired)");
                                            Ok(req.error_response(e))
                                        }
                                        Err(err) => {
                                            let e = MyProblem::AuthBiscuitLookupError;
                                            error!("{e}: {err}");
                                            Ok(req.error_response(e))
                                        }
                                    }
                                })
                            }
                            Err(biscuit_err) => {
                                let e = MyProblem::AuthInvalidBiscuit;
                                        debug!("{e}: {biscuit_err}");
                                        Box::pin(ready(Ok(req.error_response(e))))
                            }
                        }
                    }
                    Err(_) => {
                        let e = MyProblem::AuthInvalidAuthorizationHeader;
                        debug!("{e}");
                        Box::pin(ready(Ok(req.error_response(e))))
                    }
                }
            }
            None => {
                let e = MyProblem::AuthNoAuthorizationHeader;
                debug!("{e}");
                Box::pin(ready(Ok(req.error_response(e))))
            }
        }
    }
}
