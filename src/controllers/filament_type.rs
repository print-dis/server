use actix_web::{
    error, get, post,
    web::{self, Data},
    HttpResponse, Responder,
};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::models::filament_type::{FilamentType, NewFilamentType};
use crate::schema::filament_types::dsl::filament_types;

#[utoipa::path(
responses(
(status = 200, description = "List known filament types", body = [FilamentType])
)
)]
#[get("/filamenttype")]
pub async fn get_filamenttype(
    pool: Data<crate::models::db::DbPool>,
) -> actix_web::Result<impl Responder> {
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

#[utoipa::path(
    post,
    request_body = NewFilamentType,
    responses(
        (status = 201, description = "Add new filament type", body = [FilamentType])
    )
)]
#[post("/filamenttype")]
pub async fn post_filamenttype(
    info: web::Json<NewFilamentType>,
    pool: Data<crate::models::db::DbPool>,
) -> actix_web::Result<impl Responder> {
    let req = info.into_inner();

    let created_type = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        diesel::insert_into(crate::schema::filament_types::table)
            .values(&req)
            .returning(FilamentType::as_returning())
            .get_result(&mut conn)
            .expect("Error saving new filament type")
    })
    .await?;

    Ok(HttpResponse::Created().json(created_type))
}
