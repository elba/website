CREATE MATERIALIZED VIEW ts_vectors AS
    SELECT
        package_groups.package_group_name,
        packages.package_name,
        versions.semver,
        setweight(to_tsvector(package_group_name_origin), 'A') ||
        setweight(to_tsvector(package_name_origin), 'A') ||
        setweight(to_tsvector(coalesce(description, '')), 'B') ||
        setweight(to_tsvector(coalesce(textfile, '')), 'C') AS document
    FROM
        package_groups
            JOIN packages ON package_groups.id = packages.package_group_id
            JOIN versions ON packages.id = versions.package_id
            LEFT JOIN readmes ON versions.id = readmes.version_id;

CREATE INDEX ts_idx ON ts_vectors USING gin(document);
