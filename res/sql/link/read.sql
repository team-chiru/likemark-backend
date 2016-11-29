SELECT
    b.id,
    b.name,
    b.url,
    b.rev_no
FROM bookmark b
WHERE
    b.id = {{ id }}
;
