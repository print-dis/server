use diesel::prelude::*;
use crate::schema::users::id;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::filament_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FilamentType {
    pub id: i32,
    pub name: String,
    pub manufacturer: String,
    pub extruder_temperature: i32,
    pub bed_temperature: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::filaments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Filament {
    pub id: i32,
    pub color: String,
    pub notes: Option<String>,

}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Filament))]
#[diesel(belongs_to(FilamentType))]
#[diesel(table_name = crate::schema::filaments_filament_types)]
#[diesel(primary_key(filament_id, filament_type_id))]
pub struct FilamentFilamentType {
    pub filament_id: i32,
    pub filament_type_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Filament))]
#[diesel(belongs_to(Printer))]
#[diesel(table_name = crate::schema::print_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PrintRequest {
    pub id: i32,
    pub requester_id: i32,
    pub assigned_printer_id: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::printers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Printer {
    pub id: i32,
    pub name: String,
    pub bed_size_x: i32,
    pub bed_size_y: i32,
    pub print_height: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
}
