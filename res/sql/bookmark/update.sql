UPDATE bookmarkt.bookmark SET
    id = {{ id }},
    name = {{ name }},
    url = {{ url }}
WHERE
    id = {{ id }}
;
