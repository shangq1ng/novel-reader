CREATE TABLE users (
    id UUID PRIMARY KEY UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    sub TEXT NOT NULL,
    provider TEXT NOT NULL,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    display_name TEXT NOT NULL,
    avatar_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

