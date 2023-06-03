create table printers (
    id serial primary key,
    name varchar not null,
    bed_size_x int,
    bed_size_y int,
    print_height int 
);