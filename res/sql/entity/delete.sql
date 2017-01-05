DELETE FROM entity WHERE
	SUBSTR(e.parent_id, 1, LENGTH( {{ parent_id }} )) = {{ parent_id }}
;
