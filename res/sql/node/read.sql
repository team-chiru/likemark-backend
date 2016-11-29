SELECT
    n.id,
    n.name,
    n.node_id
    n.rev_no
FROM node n
WHERE
    n.id = {{ id }}
;
