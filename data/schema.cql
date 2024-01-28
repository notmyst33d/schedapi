DROP TABLE IF EXISTS groups;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS users_access_token_username_composite;
DROP TABLE IF EXISTS internal;
DROP TYPE IF EXISTS schedule;
DROP TYPE IF EXISTS range;
DROP TYPE IF EXISTS enum_value;

CREATE TYPE range (
    start int,
    end int,
);

CREATE TYPE enum_value (
    value int,
);

CREATE TYPE schedule (
    day int,
    num int,
    week_ranges list<frozen<range>>,
    name text,
    lesson_type text,
    teacher text,
    auditorium text,
    even_odd frozen<enum_value>,
);

CREATE TABLE groups (
    id uuid,
    epoch bigint,
    name text,
    schedule list<frozen<schedule>>,
    PRIMARY KEY (id),
);

CREATE TABLE users (
    username text,
    access_token uuid,
    group_scope list<uuid>,
    password text,
    PRIMARY KEY (username),
);

CREATE TABLE users_access_token_username_composite (
    access_token uuid,
    username text,
    PRIMARY KEY (access_token),
);

CREATE TABLE internal (
    id uuid,
    epoch bigint,
    PRIMARY KEY (id),
);
