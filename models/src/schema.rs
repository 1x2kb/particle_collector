// @generated automatically by Diesel CLI.

diesel::table! {
    particles (id) {
        id -> Int4,
        micro_meter_10 -> Numeric,
        micro_meter_60 -> Numeric,
        micro_meter_180 -> Numeric,
        micro_meter_500 -> Numeric,
        insert_time -> Timestamptz,
    }
}
