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
    e.id = {{ id }}
;
