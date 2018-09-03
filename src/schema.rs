table! {
    dependencies (id) {
        id -> Int4,
        version_id -> Int4,
        package_id -> Int4,
        version_req -> Varchar,
    }
}

table! {
    package_groups (id) {
        id -> Int4,
        user_id -> Int4,
        package_group_name -> Varchar,
        package_group_name_origin -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    packages (id) {
        id -> Int4,
        package_group_id -> Int4,
        package_name -> Varchar,
        package_name_origin -> Varchar,
        description -> Nullable<Varchar>,
        homepage -> Nullable<Varchar>,
        repository -> Nullable<Varchar>,
        readme_file -> Nullable<Varchar>,
        license -> Nullable<Varchar>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Nullable<Varchar>,
        gh_id -> Int4,
        gh_name -> Varchar,
        gh_access_token -> Varchar,
        gh_avatar -> Nullable<Varchar>,
        token -> Varchar,
        created_at -> Timestamp,
        last_used_at -> Timestamp,
    }
}

table! {
    version_authors (id) {
        id -> Int4,
        version_id -> Int4,
        name -> Varchar,
    }
}

table! {
    versions (id) {
        id -> Int4,
        package_id -> Int4,
        semver -> Varchar,
        yanked -> Bool,
        created_at -> Timestamp,
    }
}

joinable!(dependencies -> packages (package_id));
joinable!(dependencies -> versions (version_id));
joinable!(package_groups -> users (user_id));
joinable!(packages -> package_groups (package_group_id));
joinable!(version_authors -> versions (version_id));
joinable!(versions -> packages (package_id));

allow_tables_to_appear_in_same_query!(
    dependencies,
    package_groups,
    packages,
    users,
    version_authors,
    versions,
);
