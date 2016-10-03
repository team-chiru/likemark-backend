UPDATE bookmarkt.bookmark SET
    id = {{ id }},
    name = {{ name }},
    url = {{ url }}
    stamp = {{ stamp }},
    rev_no = {{ rev_no }}
WHERE
    id = {{ id }}
;
