CREATE TABLE IF NOT EXISTS cycle (
    id SERIAL PRIMARY KEY,
    userId int NOT NULL,
    startDate timestamp DEFAULT current_timestamp NOT NULL
);

CREATE TABLE IF NOT EXISTS event (
    id SERIAL PRIMARY KEY,
    stream_id BIGINT NOT NULL,
    version BIGINT NOT NULL,
    data JSONB NOT NULL,
     UNIQUE (stream_id, version)
)