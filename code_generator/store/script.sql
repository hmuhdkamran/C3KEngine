SELECT CONCAT(table_schema, '::', table_name, '::', table_name, '_routes,'), CONCAT('.configure(', table_name ,'_routes)')
FROM (
SELECT SUBSTRING(LOWER(regexp_replace(table_schema, '([A-Z])', '_\1', 'g')), 2, LENGTH(LOWER(regexp_replace(table_schema, '([A-Z])', '_\1', 'g')))) AS table_schema,
       SUBSTRING(LOWER(regexp_replace(table_name, '([A-Z])', '_\1', 'g')), 2, LENGTH(LOWER(regexp_replace(table_name, '([A-Z])', '_\1', 'g')))) AS table_name
FROM information_schema.tables
WHERE table_type = 'BASE TABLE' AND table_schema NOT IN('pg_catalog', 'information_schema')
ORDER BY table_schema, table_name) fnl;