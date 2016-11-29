SELECT
   l.id,
   l.name,
   l.node_id,
   l.rev_no
FROM node n
INNER JOIN link l
ON l.node_id = n.id
WHERE
 n.node_id = {{ id }}
;
