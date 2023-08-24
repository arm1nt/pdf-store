CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE pdfs (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    title TEXT,
    file_name TEXT NOT NULL UNIQUE,
    author TEXT,
    pages INTEGER,
    comments TEXT,
    time_added TIMESTAMPTZ,
    last_accessed TIMESTAMPTZ,
    picture TEXT
);


CREATE TABLE tags (
    name TEXT PRIMARY KEY
);

CREATE TABLE tags_to_pdfs (
    name TEXT,
    id uuid,
    PRIMARY KEY (name, id),
    FOREIGN KEY (id) REFERENCES pdfs(id) ON DELETE CASCADE,
    FOREIGN KEY (name) REFERENCES tags(name) ON DELETE CASCADE
);