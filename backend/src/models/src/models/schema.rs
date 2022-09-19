table! {
    users (username) {
        user_id -> Nullable<Uuid>,
        username -> Varchar,
        password -> Nullable<Varchar>,
        registerd_time -> Nullable<Timestamp>,
    }
}
