SELECT
    b.id,
    b.name,
    b.time_created,
    b.url,
    b.stamp,
    b.rev_no
FROM bookmark b
WHERE
    b.id = {{ id }}
;
