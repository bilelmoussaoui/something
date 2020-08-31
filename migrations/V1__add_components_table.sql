-- Your SQL goes here
CREATE TABLE components (
    id SERIAL PRIMARY KEY,
    app_id VARCHAR(250),
    appstream JSONB
);