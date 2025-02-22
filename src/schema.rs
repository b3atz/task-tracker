// @generated automatically by Diesel CLI.

diesel::table! {
    dot (id) {
        id -> Int4,
        date -> Int4,
        complete -> Bool,
        progress -> Nullable<Int4>,
        task_id -> Int4,
    }
}

diesel::table! {
    task (id) {
        id -> Int4,
        #[max_length = 255]
        user_id -> Varchar,
        title -> Varchar,
        body -> Nullable<Text>,
        status -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (google_id) {
        #[max_length = 255]
        google_id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        username -> Varchar,
        role -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(dot -> task (task_id));
diesel::joinable!(task -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    dot,
    task,
    users,
);
