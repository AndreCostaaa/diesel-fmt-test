// @generated automatically by Diesel CLI.

diesel::table! {
    t1 (id) {
        id -> Int4,
        t0_id -> Int4,
    }
}

diesel::table! {
    t2 (id) {
        id -> Int4,
        t1_id -> Int4,
    }
}

diesel::table! {
    t3 (id) {
        id -> Int4,
        t2_id -> Int4,
    }
}

diesel::table! {
    t4 (id) {
        id -> Int4,
        t3_id -> Int4,
    }
}

diesel::joinable!(t2 -> t1 (t1_id));
diesel::joinable!(t3 -> t2 (t2_id));
diesel::joinable!(t4 -> t3 (t3_id));

diesel::allow_tables_to_appear_in_same_query!(
    t1,
    t2,
    t3,
    t4,
);
