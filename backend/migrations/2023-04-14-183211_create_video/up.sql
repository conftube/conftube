CREATE TABLE videos
(
    "id"            VARCHAR(30)  NOT NULL,
    "platform"      VARCHAR(10)  NOT NULL,
    "title"         VARCHAR(255) NOT NULL,
    "description"   TEXT         NOT NULL,
    "published_at"  TIMESTAMPTZ  NOT NULL,
    "thumbnail_url" VARCHAR(255) NOT NULL,
    "rating"        DOUBLE PRECISION,

    CONSTRAINT "videos_pkey" PRIMARY KEY ("id")
);
