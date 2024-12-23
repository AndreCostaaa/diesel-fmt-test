// @generated automatically by Diesel CLI.

diesel::table! {
    t0 (id) {
        id -> Int4,
    }
}

diesel::table! {
    t1 (id) {
        id -> Int4,
        t0_id -> Int4,
    }
}

diesel::joinable!(t1 -> t0 (t0_id));

diesel::allow_tables_to_appear_in_same_query!(
    t0,
    t1,
);
