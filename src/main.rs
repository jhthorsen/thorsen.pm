use actix_web::{middleware::Logger, web, App, HttpServer};
use sqlx::sqlite::SqlitePool;

mod db;
mod server_error;
mod services;
mod template;

pub struct AppState {
    pub db: SqlitePool,
    pub tera: tera::Tera,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").unwrap_or("sqlite::memory:".to_string());
    let pool = db::build_db(&database_url).await.expect("Unable to setup / connect to sqlite database");
    log::info!("✅ Connected to migrated database {}", database_url);

    let port: u16 = std::env::var("PORT")
        .unwrap_or("3032".to_owned())
        .parse()
        .unwrap();

    log::info!("🚀 Listening to http://127.0.0.1:{}/ ...", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                tera: crate::template::global_tera().clone(),
            }))
            .configure(services::configure)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
