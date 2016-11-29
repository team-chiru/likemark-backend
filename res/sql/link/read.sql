SELECT
    l.id,
    l.name,
    l.url,
    l.rev_no
FROM link l
WHERE
    l.id = {{ id }}
;
