CREATE TABLE game_sessions
(
    id         bigserial PRIMARY KEY,
    session_id bigint,
    user_id    bigint,
    game_id    bigint,
    currency   bigint,
    start_time timestamp NOT NULL DEFAULT timezone('utc'::text, CURRENT_TIMESTAMP)
);

CREATE TABLE game_events
(
    id                        bigserial PRIMARY KEY,
    session_id                bigint,
    provider_id               bigint,
    game_id                   bigint,
    user_id                   bigint,
    currency                  bigint,
    spin_id                   bigint,
    balance_before            numeric(13, 4),
    balance_after             numeric(13, 4),
    bet                       numeric(13, 4),
    win                       numeric(13, 4),
    profit                    numeric(13, 4),
    transaction_create_time   timestamp NOT NULL DEFAULT timezone('utc'::text, CURRENT_TIMESTAMP),
    transaction_finished_time timestamp NOT NULL DEFAULT timezone('utc'::text, CURRENT_TIMESTAMP)
);
