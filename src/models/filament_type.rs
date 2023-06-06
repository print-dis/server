use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Serialize, Deserialize, ToSchema, Clone, Debug)]
#[diesel(table_name = crate::schema::filament_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FilamentType {
    pub id: i32,
    pub name: String,
    pub manufacturer: String,
    pub extruder_temperature: i32,
    pub bed_temperature: i32,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = crate::schema::filament_types)]
pub struct NewFilamentType {
    pub name: String,
    pub manufacturer: String,
    pub extruder_temperature: i32,
    pub bed_temperature: i32,
}
