UPDATE entity SET
    id = {{ id }},
    parent_id = {{ parent_id }}
    name = {{ name }},
    url = {{ url }},
    struct_type = {{ struct_type }}
    fn_type = {{ fn_type }}
    rev_no = {{ rev_no }}
WHERE
    id = {{ id }}
;
