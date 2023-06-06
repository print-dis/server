use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::filaments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Filament {
    pub id: i32,
    pub color: String,
    pub notes: Option<String>,
}
