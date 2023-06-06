// @generated automatically by Diesel CLI.

diesel::table! {
    filament_types (id) {
        id -> Int4,
        name -> Varchar,
        manufacturer -> Varchar,
        extruder_temperature -> Int4,
        bed_temperature -> Int4,
    }
}

diesel::table! {
    filaments (id) {
        id -> Int4,
        color -> Varchar,
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    filaments_filament_types (filament_id, filament_type_id) {
        filament_id -> Int4,
        filament_type_id -> Int4,
    }
}

diesel::table! {
    print_requests (id) {
        id -> Int4,
        requester_id -> Int4,
        assigned_printer_id -> Nullable<Int4>,
        filament_id -> Nullable<Int4>,
        request_object -> Nullable<Varchar>,
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    printers (id) {
        id -> Int4,
        name -> Varchar,
        bed_size_x -> Int4,
        bed_size_y -> Int4,
        print_height -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
    }
}

diesel::joinable!(filaments_filament_types -> filament_types (filament_type_id));
diesel::joinable!(print_requests -> printers (assigned_printer_id));
diesel::joinable!(print_requests -> users (requester_id));

diesel::allow_tables_to_appear_in_same_query!(
    filament_types,
    filaments,
    filaments_filament_types,
    print_requests,
    printers,
    users,
);
