diesel::table! {
    users (username) {
        user_id -> Varchar,
        username -> Varchar,
        password -> Varchar,
    }
}
