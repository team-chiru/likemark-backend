SELECT
	b.id,
	b.name,
	b.type
FROM  bookmarkt.bookmark b
WHERE
	( {{ type }} IS NULL OR {{ type }} = b.type) AND
	( {{ name }} IS NULL OR b.name = {{ name }})
;
