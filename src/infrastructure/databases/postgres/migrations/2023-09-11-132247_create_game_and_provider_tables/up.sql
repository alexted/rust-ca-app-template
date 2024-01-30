CREATE TABLE games
(
    id          SERIAL PRIMARY KEY,
    provider_id int4      NOT NULL,
    code        varchar   NOT NULL,
    "name"      varchar   NOT NULL,
    image       varchar   NOT NULL,
    is_mobile   bool      NOT NULL,
    is_desktop  bool      NOT NULL,
    is_tablet   bool      NOT NULL,
    is_demo     bool      NOT NULL,
    is_embedded bool      NOT NULL,
    bonus       int4      NOT NULL,
    free_spins  int4      NOT NULL,
    is_active   bool      NOT NULL DEFAULT true,
    created_at  timestamp NOT NULL DEFAULT timezone('utc'::text, CURRENT_TIMESTAMP),
    updated_at  timestamp NULL
);



CREATE TABLE providers
(
    id         SERIAL PRIMARY KEY,
    "name"     varchar   NOT NULL,
    aggregator varchar   NOT NULL,
    created_at timestamp NOT NULL DEFAULT timezone('utc'::text, CURRENT_TIMESTAMP),
    updated_at timestamp NULL
);