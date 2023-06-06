use diesel::prelude::*;

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
