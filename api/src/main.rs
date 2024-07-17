use std::{str::FromStr, time::Duration};
use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{middleware::{self, Logger, NormalizePath}, App, web, HttpServer};
use biscuit_auth::{KeyPair, PrivateKey};

use clap::{crate_name, Parser};
use lettre::Address;
use log::{info, warn};
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, PgPool};
use url::Url;

use crate::auth::middleware_biscuit;

mod auth;
mod users_settings;
mod utils;

const APP_TITLE: &str = "Thomas's template";
const WEBAPP_INDEX_FILE: &str = "index.html";
#[derive(Debug, Clone, Parser)]
#[clap(author, about, version, name = APP_TITLE)]
struct Config {
    #[clap(long, env, hide_env_values = true, value_parser = parse_biscuit_private_key)]
    biscuit_private_key: Option<PrivateKey>,

    #[clap(long, env, default_value = "10")]
    max_db_connections: u32,

    #[clap(long, env, hide_env_values = true)]
    database_url: String,

    #[clap(long, env, default_value = "true")]
    auto_db_migration: bool,

    #[clap(long, env, default_value = "true")]
    enable_security_headers: bool,

    #[clap(long, env, default_value = "true")]
    enable_hsts_header: bool,

    #[clap(long, env, default_value = "http://localhost:8080")]
    cors_allowed_origins: Vec<String>,

    #[clap(long, env, default_value = "127.0.0.1")]
    ip: String,

    #[clap(long, env, default_value = "8080")]
    port: String,

    #[clap(long, env, default_value = "12")]
    password_minimum_length: u8,

    /// Sender email address
    #[clap(long, env)]
    email_sender_address: Address,

    /// Sender name
    #[clap(long, env, default_value = "Hook0")]
    email_sender_name: String,

    /// Connection URL to SMTP server; for example: `smtp://localhost:1025`, `smtps://user:password@provider.com:465` (SMTP over TLS) or `smtp://user:password@provider.com:465?tls=required` (SMTP with STARTTLS)
    #[clap(long, env, hide_env_values = true)]
    smtp_connection_url: String,

    /// Duration (in second) to use as timeout when sending emails to the SMTP server
    #[clap(long, env, default_value = "5")]
    smtp_timeout_in_s: u64,

    /// URL of the Hook0 logo
    #[clap(long, env, default_value = "https://hook0.com/256x256.png")]
    email_logo_url: Url,

    /// Frontend application URL (used for building links in emails)
    #[clap(long, env)]
    app_url: Url,

    /// Api URL
    #[clap(long, env)]
    api_url: Url,

    /// Disable serving the frontend
    #[clap(long, env, default_value = "false")]
    disable_serving_webapp: bool,

    /// Path to the directory containing the web app to serve
    #[clap(long, env, default_value = "../frontend/dist/")]
    webapp_path: String,

    /// Path to the profile picture directory
    #[clap(long, env, default_value = "../frontend/public/profile-pictures/")]
    profile_picture_dir: String,
}

#[derive(Debug, Clone)]
struct State {
    db: PgPool,
    biscuit_private_key: PrivateKey,
    password_minimum_length: u8,
    mailer: utils::mailer::Mailer,
    app_url: Url,
    profile_picture_dir: String,
}

fn parse_biscuit_private_key(input: &str) -> Result<PrivateKey, String> {
    PrivateKey::from_bytes_hex(input)
        .map_err(|e| format!("Value of BISCUIT_PRIVATE_KEY is invalid ({e}). Re-run this app without the environment variable set to get a randomly generated key."))
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let _logger = init_logger();

    if let Some(biscuit_private_key) = config.biscuit_private_key {
        // Create a DB connection pool
        let pool = PgPoolOptions::new()
            .max_connections(config.max_db_connections)
            .connect_with(
                PgConnectOptions::from_str(&config.database_url)?.application_name(crate_name!()),
            )
            .await?;
        info!(
            "Started a pool of maximum {} DB connections",
            &config.max_db_connections
        );

        // Run migrations
        if config.auto_db_migration {
            info!("Checking/running DB migrations");
            sqlx::migrate!("./migrations").run(&pool).await?;
        }

        // Create Mailer
        let mailer = utils::mailer::Mailer::new(
            &config.smtp_connection_url,
            Duration::from_secs(config.smtp_timeout_in_s),
            config.email_sender_name,
            config.email_sender_address,
            config.email_logo_url,
        )
        .await
        .expect("Could not initialize mailer; check SMTP configuration");

        // Application state
        let initial_state = State {
            db: pool,
            biscuit_private_key: biscuit_private_key,
            password_minimum_length: config.password_minimum_length,
            mailer,
            app_url: config.app_url,
            profile_picture_dir: config.profile_picture_dir,
        };

        // Run web server
        let webapp_path = config.webapp_path.clone();
        HttpServer::new(move || {

            // Prepare CORS configuration
            let cors = {
                let mut c = Cors::default()
                    .allowed_headers([
                        http::header::ACCEPT,
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .max_age(3600)
                    .supports_credentials(); // Added for cookie support

                /* for origin in &config.cors_allowed_origins {
                    c = c.allowed_origin(origin);
                } */
                
                c = c.allow_any_origin(); // Allow all origins for now
                c
            };

            // Prepare auth middleware
            let biscuit_auth = middleware_biscuit::BiscuitAuth {
                db: initial_state.db.clone(),
                biscuit_private_key: initial_state.biscuit_private_key.clone(),
            };
            
            let security_headers = middleware::DefaultHeaders::new()
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
                .add(("X-XSS-Protection", "1; mode=block"))
                .add(("Referrer-Policy", "SAMEORIGIN"));

            let hsts_header = middleware::DefaultHeaders::new()
                .add(("Strict-Transport-Security", "max-age=157680000"));

            let security_headers_condition =
                middleware::Condition::new(config.enable_security_headers, security_headers);

            let hsts_header_condition =
                middleware::Condition::new(config.enable_hsts_header, hsts_header);

            let mut app = App::new()
                .app_data(actix_web::web::Data::new(initial_state.clone()))
                .wrap(security_headers_condition)
                .wrap(hsts_header_condition)
                .wrap(cors)
                .wrap(Logger::default())
                .wrap(NormalizePath::trim())
                .service(
                    
                    web::scope("/api")
                        .service(
                            web::scope("/v1")
                                .service(
                                    web::scope("/auth")
                                        .service(
                                            web::resource("/login")
                                                .route(web::post().to(auth::auth::login)),
                                        )
                                        .service(
                                            web::resource("/register")
                                                .route(web::post().to(auth::registration::register)),
                                        )
                                        .service(
                                            web::resource("/logout")
                                                .wrap(biscuit_auth.clone())
                                                .route(web::post().to(auth::auth::logout)),
                                        )
                                        .service(
                                            web::resource("/refresh")
                                                .wrap(biscuit_auth.clone())
                                                .route(web::post().to(auth::auth::refresh)),
                                        )
                                        .service(
                                            web::resource("/verify-email")
                                                .route(web::post().to(auth::auth::verify_email)),
                                        ).service(
                                            web::resource("/resend-verification-email")
                                                .route(web::post().to(auth::auth::resend_email_verification)),
                                        )
                                        .service(
                                            web::resource("/begin-reset-password").route(
                                                web::post().to(auth::auth::begin_reset_password),
                                            ),
                                        )
                                        .service(
                                            web::resource("/reset-password")
                                                .route(web::post().to(auth::auth::reset_password)),
                                        )
                                        .service(
                                            web::resource("/password")
                                                .wrap(biscuit_auth.clone())
                                                .route(web::post().to(auth::auth::change_password)),
                                        ),
                                )
                                .service(
                                    web::scope("/user")
                                        .service(
                                            web::resource("/profile-picture")
                                                .wrap(biscuit_auth.clone())
                                                .route(web::post().to(users_settings::main::change_profile_picture)),
                                        )
                                        .service(
                                            web::scope("/profile")
                                                .service(
                                                    web::resource("/name")
                                                        .wrap(biscuit_auth.clone())
                                                        .route(web::post().to(users_settings::main::change_name)),
                                                )
                                        )
                                    .wrap(biscuit_auth.clone())
                                    .route("", web::delete().to(users_settings::main::delete_user)),
                                ),
                                
                        )
                );

                if !config.disable_serving_webapp {
                    app = app.default_service(
                        Files::new("/", webapp_path.as_str())
                            .index_file(WEBAPP_INDEX_FILE)
                            .default_handler(
                                NamedFile::open(format!("{}/{}", &webapp_path, WEBAPP_INDEX_FILE))
                                    .expect("Cannot open SPA main file"),
                            ),
                    );
                }

                app
        })
            .bind(&format!("{}:{}", config.ip, config.port))?
            .run()
            .await
            .map_err(|e| e.into())
    } else {
        warn!("No BISCUIT_PRIVATE_KEY environment variable found. Generating a new keypair.");
        let keypair = KeyPair::new();
        Ok(println!("BISCUIT_PRIVATE_KEY={:?}", keypair.private().to_bytes_hex()))
    }
}


/// Initialise a logger with default level at INFO
fn mk_log_builder() -> env_logger::Builder {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
}

fn init_logger() {
    mk_log_builder().init();
    info!("Started logger");
}

