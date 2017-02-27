SELECT * FROM entity e WHERE (
   ({{ id }} IS NULL OR e.id = {{ id }}) AND
   ({{ tree_id }} IS NULL OR SUBSTR(e.tree_id, 1, LENGTH( {{ tree_id }} )) = {{ tree_id }})
);
