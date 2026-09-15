CREATE TABLE chapters (
    id UUID PRIMARY KEY UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    novel_id UUID NOT NULL REFERENCES novels(id),
    chapter_number NUMERIC(10,2),
    title TEXT NOT NULL,
    content_key TEXT NOT NULL,
    word_count INT,
    published_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX chapters_idx ON chapters(novel_id, chapter_number);