UPDATE entity SET
    id = {{ id }},
    parent_id = {{ parent_id }}
    name = {{ name }},
    url = {{ url }},
    struct_type = {{ struct_type }},
    fn_type = {{ fn_type }},
    rev_no = {{ rev_no }}
WHERE
  ( {{ id }} IS NULL OR id = {{ id }} ) AND
  ( {{ parent_id }} IS NULL OR SUBSTR(parent_id, 1, LENGTH( {{ parent_id }} )) = {{ parent_id }} )
;
