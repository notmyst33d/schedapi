CREATE TYPE enum_value (
    value int,
);

CREATE TABLE internal (
    id uuid,
    epoch bigint,
    PRIMARY KEY (id),
);

INSERT INTO internal (id, epoch) VALUES (00000000-0000-0000-0000-000000000000, 0);

ALTER TABLE groups ADD epoch bigint;
ALTER TABLE groups DROP schedule;

DROP TYPE schedule;

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

ALTER TABLE groups ADD schedule2 list<frozen<schedule>>;
