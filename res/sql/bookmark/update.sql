UPDATE bookmarkt.bookmark SET
    id = {{ id }},
    name = {{ name }},
    type = {{ type }}
WHERE
    id = {{ id }}
;
