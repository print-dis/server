pub mod models;
pub mod schema;

use std::sync::Mutex;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use crate::models::FilamentType;
use  utoipa::{OpenApi};
use utoipa_swagger_ui::SwaggerUi;

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
    schemas(models::FilamentType)
    ),
    tags(
    (name = "print-dis", description = "Print job management")
    ),
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();
    
    HttpServer::new(move || {
        App::new()
        .service(hello)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
        
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Default)]
struct FilamentTypeStore {
    filament_types: Mutex<Vec<FilamentType>>    
}

#[utoipa::path(
responses(
(status = 200, description = "List known filament types", body = [FilamentType])
)
)]
#[get("/filamenttype")]
async fn get_filamenttype(filamenttype_store: Data<FilamentTypeStore>) -> impl Responder {
    let  filaments = filamenttype_store.filament_types.lock().unwrap();

    HttpResponse::Ok().json(filaments.clone())
}
