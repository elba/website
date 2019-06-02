table! {
    access_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
        created_at -> Timestamp,
        last_used_at -> Nullable<Timestamp>,
    }
}

table! {
    dependencies (id) {
        id -> Int4,
        version_id -> Int4,
        package_id -> Int4,
        version_req -> Varchar,
    }
}

table! {
    groups (id) {
        id -> Int4,
        group_name -> Varchar,
        group_name_origin -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    package_owners (id) {
        id -> Int4,
        package_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    packages (id) {
        id -> Int4,
        group_id -> Int4,
        package_name -> Varchar,
        package_name_origin -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        gh_id -> Int4,
        gh_name -> Varchar,
        gh_access_token -> Varchar,
        gh_avatar -> Nullable<Varchar>,
        created_at -> Timestamp,
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
    version_downloads (id) {
        id -> Int4,
        version_id -> Int4,
        date -> Date,
        downloads -> Int4,
    }
}

table! {
    version_keywords (id) {
        id -> Int4,
        version_id -> Int4,
        keyword -> Varchar,
    }
}

table! {
    versions (id) {
        id -> Int4,
        package_id -> Int4,
        semver -> Varchar,
        yanked -> Bool,
        description -> Nullable<Varchar>,
        homepage -> Nullable<Varchar>,
        repository -> Nullable<Varchar>,
        license -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

joinable!(access_tokens -> users (user_id));
joinable!(dependencies -> packages (package_id));
joinable!(dependencies -> versions (version_id));
joinable!(groups -> users (user_id));
joinable!(package_owners -> packages (package_id));
joinable!(package_owners -> users (user_id));
joinable!(packages -> groups (group_id));
joinable!(version_authors -> versions (version_id));
joinable!(version_downloads -> versions (version_id));
joinable!(version_keywords -> versions (version_id));
joinable!(versions -> packages (package_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    dependencies,
    groups,
    package_owners,
    packages,
    users,
    version_authors,
    version_downloads,
    version_keywords,
    versions,
);
