SELECT
	b.id,
	b.name,
	b.url
FROM  bookmarkt.bookmark b
WHERE
	( {{ url }} IS NULL OR b.url = {{ url }}) AND
	( {{ name }} IS NULL OR b.name = {{ name }})
;
