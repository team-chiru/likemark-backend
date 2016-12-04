SELECT
    n.id,
    n.name,
    n.type,
    n.node_id,
    n.rev_no
FROM  node n
WHERE
    ( {{ name }} IS NULL OR n.name = {{ name }}) AND
    ( {{ type }} IS NULL OR n.type = {{ type }}) AND
    ( {{ node_id }} IS NULL OR n.node_id = {{ node_id }})
;
