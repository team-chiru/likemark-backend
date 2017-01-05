UPDATE entity SET
    id = {{ id }},
    parent_id = {{ parent_id }}
    name = {{ name }},
    url = {{ url }},
    struct_type = {{ struct_type }}
    fn_type = {{ fn_type }}
    rev_no = {{ rev_no }}
WHERE
  ( {{ e.id }} IS NULL OR e.id = {{ id }} ) AND
  ( {{ e.parent_id }} IS NULL OR SUBSTR(e.parent_id, 1, LENGTH( {{ parent_id }} )) = {{ parent_id }} )
;
