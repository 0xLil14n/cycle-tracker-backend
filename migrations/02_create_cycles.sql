CREATE TABLE IF NOT EXISTS cycle (
    id SERIAL PRIMARY KEY,
    userId int NOT NULL,
    startDate timestamp DEFAULT current_timestamp NOT NULL
);

CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    stream_id bigint NOT NULL,
    version bigint NOT NULL,
    data jsonb NOT NULL,
    UNIQUE (stream_id, version)
)

CREATE TABLE IF NOT EXISTS event_queue (
    id SERIAL PRIMARY KEY,
    event_id INT NOT NULL REFERENCES events(id)
)

-- Events {
--     PeriodStarted -> stream_id = cycle // creates a new cycle, ends prev cycle
--     PeriodEnded -> stream_id = cycle
--     SymptomLogged -> stream_id = cycle
--     FactorLogged -> stream_id = cycle
-- }