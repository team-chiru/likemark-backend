SELECT
    l.id,
    l.name,
    l.url,
    l.rev_no
FROM  link l
WHERE
    ( {{ url }} IS NULL OR l.url = {{ url }}) AND
    ( {{ name }} IS NULL OR l.name = {{ name }})
;
