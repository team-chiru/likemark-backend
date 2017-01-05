DELETE FROM entity WHERE
	( {{ id }} IS NULL OR e.id = {{ id }} ) AND
	( {{ parent_id }} IS NULL OR SUBSTR(e.parent_id, 1, LENGTH( {{ parent_id }} )) = {{ parent_id }} )
;
