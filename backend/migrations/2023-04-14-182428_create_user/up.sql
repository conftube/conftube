CREATE TABLE users
(
    "id"          SERIAL       NOT NULL,
    "email"       VARCHAR(70)  NOT NULL,
    "family_name" VARCHAR(70)  NOT NULL,
    "given_name"  VARCHAR(70)  NOT NULL,
    "picture"     VARCHAR(255) NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "users_email_unique" UNIQUE ("email")
);
