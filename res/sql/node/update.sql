UPDATE node SET
    id = {{ id }},
    name = {{ name }},
    node_id = {{ node_id }},
    rev_no = {{ rev_no }}
WHERE
    id = {{ id }}
;
