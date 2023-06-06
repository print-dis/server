use std::env;

use actix_web::web::Data;
use actix_web::{error, get, put, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use models::filament_type::NewFilamentType;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use dotenvy::dotenv;

use crate::models::filament_type::FilamentType;
use crate::schema::filament_types::dsl::filament_types;

pub mod models;
pub mod schema;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
    paths(
    get_filamenttype
    ),
    components(
    schemas(FilamentType)
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
            .service(hello)
            .service(get_filamenttype)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[utoipa::path(
responses(
(status = 200, description = "List known filament types", body = [FilamentType])
)
)]
#[get("/filamenttype")]
async fn get_filamenttype(pool: Data<models::db::DbPool>) -> actix_web::Result<impl Responder> {
    let filament_type = web::block(move || {
        // Obtaining a connection from the pool is also a potentially blocking operation.
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        filament_types
            .select(FilamentType::as_select())
            .load(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(filament_type))
}


    Ok(HttpResponse::Created().json(created_type))
}
