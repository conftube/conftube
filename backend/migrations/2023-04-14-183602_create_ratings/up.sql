CREATE TABLE ratings
(
    "id"       SERIAL           NOT NULL,
    "video_id" VARCHAR(30)      NOT NULL,
    "user_id"  INTEGER          NOT NULL,
    "rating"   DOUBLE PRECISION NOT NULL,

    CONSTRAINT "ratings_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "ratings_video_key" FOREIGN KEY ("video_id") REFERENCES videos ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "ratings_user_key" FOREIGN KEY ("user_id") REFERENCES users ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "rating_video_user_unique_key" UNIQUE ("video_id", "user_id")
);
