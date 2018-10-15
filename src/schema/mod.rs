mod schema;

pub use self::schema::*;

table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector;

    ts_vectors (id) {
        id -> Int4,
        group_name -> Varchar,
        package_name -> Varchar,
        semver -> Varchar,
        document -> TsVector,
    }
}
