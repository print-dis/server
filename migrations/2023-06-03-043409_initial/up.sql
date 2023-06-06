create table printers
(
    id           serial primary key,
    name         varchar not null,
    bed_size_x   int     not null,
    bed_size_y   int     not null,
    print_height int     not null
);

create table filament_types
(
    id                   serial primary key,
    name                 varchar not null,
    manufacturer         varchar not null,
    extruder_temperature int     not null,
    bed_temperature      int     not null
);

create table filaments
(
    id    serial primary key,
    color varchar not null,
    notes varchar
);

create table filaments_filament_types
(
    filament_id      int not null,
    filament_type_id int not null,
    constraint fk_filament_id
        foreign key (filament_id)
            references filaments (id)
            on update cascade
            on delete restrict,

    constraint fk_filament_type_id
        foreign key (filament_type_id)
            references filament_types (id)
            on update cascade
            on delete restrict,

    primary key (filament_id, filament_type_id)
);

create table users
(
    id serial primary key
    -- placeholder
);

create table print_requests
(
    id                  serial primary key,
    requester_id        int not null,
    assigned_printer_id int,
    filament_id         int,
    request_object      varchar,
    notes               varchar,

    CONSTRAINT fk_requester_id
        foreign key (requester_id)
            references users (id)
            on update cascade,
            
            CONSTRAINT fk_assigned_printer_id
            foreign key (assigned_printer_id)
            references printers (id)
            on update cascade,
            
    CONSTRAINT fk_filament_id
        foreign key (filament_id)
            references filaments (id)
            on update cascade
);