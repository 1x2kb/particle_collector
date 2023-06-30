-- Your SQL goes here
CREATE TABLE particles (
    id SERIAL PRIMARY KEY,
    micro_meter_10 NUMERIC NOT NULL,
    micro_meter_60 NUMERIC NOT NULL,
    micro_meter_180 NUMERIC NOT NULL,
    micro_meter_500 NUMERIC NOT NULL,
    insert_time TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
)