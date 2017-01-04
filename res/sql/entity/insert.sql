INSERT INTO entity
	( id, parent_id, name, url, struct_type, fn_type, rev_no)
VALUES
	(
		{{ id }},
		{{ parent_id }},
		{{ name }},
	  {{ url }},
    {{ struct_type }},
	  {{ fn_type }},
	  {{ rev_no }}
  )
;
