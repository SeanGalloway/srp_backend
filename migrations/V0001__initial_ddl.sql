create table users(
     id serial primary key,
     name_first varchar(50),
     name_last varchar(80),
     email varchar(80) not null,
     role varchar(15) not null,
     password text not null,
     salt text not null,
     created_ts timestamp not null default CURRENT_TIMESTAMP
);

create table note(
    id uuid primary key not null default gen_random_uuid(),
    owner_id integer not null,
    title varchar(100),
    body text,
    created_ts timestamp not null default CURRENT_TIMESTAMP
)