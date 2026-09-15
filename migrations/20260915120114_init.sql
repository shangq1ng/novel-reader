CREATE TABLE novels (
    id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(62) UNIQUE NOT NULL,
    cover_url TEXT,
    author TEXT NOT NULL UNIQUE,
    synopsis TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);