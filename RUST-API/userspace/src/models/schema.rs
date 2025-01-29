// @generated automatically by Diesel CLI.

diesel::table! {
    permissions (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    role_permissions (id) {
        id -> Int4,
        role_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    user_ips (id) {
        id -> Int4,
        user_id -> Uuid,
        ip_address -> Inet,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_permissions (id) {
        id -> Int4,
        user_id -> Uuid,
        permission_id -> Int4,
        value -> Nullable<Bool>,
    }
}

diesel::table! {
    user_profiles (id) {
        id -> Int4,
        user_id -> Uuid,
        #[max_length = 100]
        display_name -> Nullable<Varchar>,
        profile_picture -> Nullable<Text>,
        bio -> Nullable<Text>,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Int4,
        user_id -> Uuid,
        role_id -> Int4,
    }
}

diesel::table! {
    user_socials (id) {
        id -> Int4,
        user_id -> Uuid,
        #[max_length = 50]
        platform -> Varchar,
        url -> Text,
    }
}

diesel::table! {
    user_stats (user_id) {
        user_id -> Uuid,
        created_at -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
        last_activity -> Nullable<Timestamp>,
        last_login_ip -> Nullable<Inet>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        password_hash -> Nullable<Text>,
        #[max_length = 10]
        auth_type -> Varchar,
        #[max_length = 50]
        sso_provider -> Nullable<Varchar>,
        #[max_length = 10]
        user_status -> Nullable<Varchar>,
        #[max_length = 10]
        theme_preference -> Nullable<Varchar>,
    }
}

diesel::joinable!(role_permissions -> permissions (permission_id));
diesel::joinable!(role_permissions -> roles (role_id));
diesel::joinable!(user_ips -> users (user_id));
diesel::joinable!(user_permissions -> permissions (permission_id));
diesel::joinable!(user_permissions -> users (user_id));
diesel::joinable!(user_profiles -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_socials -> users (user_id));
diesel::joinable!(user_stats -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    permissions,
    role_permissions,
    roles,
    user_ips,
    user_permissions,
    user_profiles,
    user_roles,
    user_socials,
    user_stats,
    users,
);
