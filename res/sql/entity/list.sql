SELECT
    e.id,
    e.tree_id,
    e.name,
    e.url,
    e.struct_type,
    e.fn_type,
    e.rev_no
FROM  entity e
WHERE
    ( {{ url }} IS NULL OR e.url = {{ url }} ) AND
    ( {{ name }} IS NULL OR e.name = {{ name }} ) AND
    ( {{ struct_type }} IS NULL OR e.struct_type = {{ struct_type }} ) AND
    ( {{ fn_type }} IS NULL OR e.fn_type = {{ fn_type }} ) AND
    ( {{ rev_no }} IS NULL OR e.rev_no = {{ rev_no }} )
;
