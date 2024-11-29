SELECT 
    CONCAT(table_schema, '::', table_name, '::', table_name, '_routes,') AS route_config,
    CONCAT('.configure(', table_name, '_routes)') AS configure_statement,
    CONCAT(table_schema, '-', table_name) AS schema_table,
    regexp_replace(
        regexp_replace(orignal_table_name, '([A-Z])', ' \1', 'g'), 
        '^ ', 
        ''
    ) AS formatted_table_name
FROM (
    SELECT 
        SUBSTRING(LOWER(regexp_replace(table_schema, '([A-Z])', '_\1', 'g')), 2, LENGTH(LOWER(regexp_replace(table_schema, '([A-Z])', '_\1', 'g')))) AS table_schema,
        SUBSTRING(LOWER(regexp_replace(table_name, '([A-Z])', '_\1', 'g')), 2, LENGTH(LOWER(regexp_replace(table_name, '([A-Z])', '_\1', 'g')))) AS table_name,
        table_name AS orignal_table_name
    FROM 
        information_schema.tables
    WHERE 
        table_type = 'BASE TABLE' 
        AND table_schema NOT IN('pg_catalog', 'information_schema')
    ORDER BY 
        table_schema, table_name
) fnl;
