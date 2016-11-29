SELECT
    b.id,
    b.name,
    b.url,
    b.rev_no
FROM  bookmark b
WHERE
    ( {{ url }} IS NULL OR b.url = {{ url }}) AND
    ( {{ name }} IS NULL OR b.name = {{ name }})
;
