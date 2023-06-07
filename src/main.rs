use std::env;


use actix_web::{web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection};
use models::filament_type::NewFilamentType;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use dotenvy::dotenv;

use crate::controllers::filament_type::*;

use crate::models::filament_type::FilamentType;


pub mod controllers;
pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
    paths(
    get_filamenttype,
    post_filamenttype
    ),
    components(
    schemas(FilamentType, NewFilamentType)
    ),
    tags(
    (name = "print-dis", description = "Print job management")
    ),
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid Postgres URI");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_filamenttype)
            .service(post_filamenttype)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
