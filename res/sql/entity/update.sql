/* need to update all children TOO! */
UPDATE entity SET
    id = {{ id }},
    tree_id = {{ tree_id }}
    name = {{ name }},
    url = {{ url }},
    struct_type = {{ struct_type }},
    fn_type = {{ fn_type }},
    rev_no = {{ rev_no }}
WHERE (
   {{ id }} IS NULL OR id = {{ id }}
);
