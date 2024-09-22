// @generated automatically by Diesel CLI.

diesel::table! {
    course (id) {
        id -> Int4,
        teacher_id -> Int4,
        #[max_length = 140]
        name -> Varchar,
        time -> Nullable<Timestamp>,
        #[max_length = 2000]
        description -> Nullable<Varchar>,
        #[max_length = 30]
        format -> Nullable<Varchar>,
        #[max_length = 300]
        structure -> Nullable<Varchar>,
        #[max_length = 30]
        duration -> Nullable<Varchar>,
        price -> Nullable<Int4>,
        #[max_length = 30]
        language -> Nullable<Varchar>,
        #[max_length = 30]
        level -> Nullable<Varchar>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    teacher (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 200]
        picture_url -> Nullable<Varchar>,
        #[max_length = 2000]
        profile -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    course,
    posts,
    teacher,
);
