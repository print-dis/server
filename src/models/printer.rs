use diesel::prelude::*;

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
