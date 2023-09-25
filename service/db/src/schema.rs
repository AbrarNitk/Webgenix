// @generated automatically by Diesel CLI.

diesel::table! {
    auth_group (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    auth_group_permissions (id) {
        id -> Int8,
        group_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    auth_permission (id) {
        id -> Int4,
        name -> Text,
        content_type_id -> Int4,
        codename -> Text,
    }
}

diesel::table! {
    auth_user (id) {
        id -> Int4,
        password -> Text,
        last_login -> Nullable<Timestamptz>,
        is_superuser -> Bool,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        is_staff -> Bool,
        is_active -> Bool,
        date_joined -> Timestamptz,
    }
}

diesel::table! {
    auth_user_groups (id) {
        id -> Int8,
        user_id -> Int4,
        group_id -> Int4,
    }
}

diesel::table! {
    auth_user_user_permissions (id) {
        id -> Int8,
        user_id -> Int4,
        permission_id -> Int4,
    }
}

diesel::table! {
    django_admin_log (id) {
        id -> Int4,
        action_time -> Timestamptz,
        object_id -> Nullable<Text>,
        object_repr -> Text,
        action_flag -> Int2,
        change_message -> Text,
        content_type_id -> Nullable<Int4>,
        user_id -> Int4,
    }
}

diesel::table! {
    django_content_type (id) {
        id -> Int4,
        app_label -> Text,
        model -> Text,
    }
}

diesel::table! {
    django_migrations (id) {
        id -> Int8,
        app -> Text,
        name -> Text,
        applied -> Timestamptz,
    }
}

diesel::table! {
    django_session (session_key) {
        session_key -> Text,
        session_data -> Text,
        expire_date -> Timestamptz,
    }
}

diesel::joinable!(auth_group_permissions -> auth_group (group_id));
diesel::joinable!(auth_group_permissions -> auth_permission (permission_id));
diesel::joinable!(auth_permission -> django_content_type (content_type_id));
diesel::joinable!(auth_user_groups -> auth_group (group_id));
diesel::joinable!(auth_user_groups -> auth_user (user_id));
diesel::joinable!(auth_user_user_permissions -> auth_permission (permission_id));
diesel::joinable!(auth_user_user_permissions -> auth_user (user_id));
diesel::joinable!(django_admin_log -> auth_user (user_id));
diesel::joinable!(django_admin_log -> django_content_type (content_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    auth_group,
    auth_group_permissions,
    auth_permission,
    auth_user,
    auth_user_groups,
    auth_user_user_permissions,
    django_admin_log,
    django_content_type,
    django_migrations,
    django_session,
);
