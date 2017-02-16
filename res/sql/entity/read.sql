SELECT * FROM entity e WHERE
   ( {{ tree_id }} IS NULL OR SUBSTR(e.tree_id, 1, LENGTH( {{ tree_id }} )) = {{ tree_id }} )
;
