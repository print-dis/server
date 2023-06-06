use crate::models::*;
use diesel::prelude::*;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(filament::Filament))]
#[diesel(belongs_to(filament_type::FilamentType))]
#[diesel(table_name = crate::schema::filaments_filament_types)]
#[diesel(primary_key(filament_id, filament_type_id))]
pub struct FilamentFilamentType {
    pub filament_id: i32,
    pub filament_type_id: i32,
}
