INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES
	(
		{{ parent_id }},
		{{ name }},
	  {{ url }},
    {{ struct_type }},
	  {{ fn_type }},
	  {{ rev_no }}
  )
;
