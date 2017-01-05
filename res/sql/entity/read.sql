SELECT
    e.id,
    e.parent_id,
    e.name,
    e.url,
    e.struct_type,
    e.fn_type,
    e.rev_no
FROM entity e
WHERE
    ( {{ id }} IS NULL OR e.id = {{ id }} ) AND
    ( {{ parent_id }} IS NULL OR SUBSTR(e.parent_id, 1, LENGTH( {{ parent_id }} )) = {{ parent_id }} )
;
