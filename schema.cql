USE schedule_dogfood;

DROP TABLE IF EXISTS groups;
DROP TYPE IF EXISTS schedule;
DROP TYPE IF EXISTS range;

CREATE TYPE range (
    start int,
    end int,
);

CREATE TYPE schedule (
    day int,
    num int,
    week_range frozen<range>,
    name text,
    lesson_type text,
    teacher text,
    auditorium text,
    even boolean,
    odd boolean,
);

CREATE TABLE groups (
    id uuid,
    name text,
    schedule list<frozen<schedule>>,
    PRIMARY KEY (id),
);

CREATE TABLE groups_name_id_composite (
    name text,
    id uuid,
    PRIMARY KEY (name, id),
);
