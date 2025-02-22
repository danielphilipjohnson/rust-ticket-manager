// @generated automatically by Diesel CLI.

diesel::table! {
    issues (id) {
        id -> Int4,
        project_id -> Int4,
        #[max_length = 200]
        title -> Varchar,
        description -> Text,
        #[max_length = 100]
        created_by -> Varchar,
        #[max_length = 100]
        assigned_to -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_open -> Bool,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(issues -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(issues, projects,);
