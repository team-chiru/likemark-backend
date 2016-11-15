UPDATE bookmark SET
    id = {{ id }},
    name = "{{ name }}",
    url = "{{ url }}",
    rev_no = {{ rev_no }}
WHERE
    id = {{ id }}
;
