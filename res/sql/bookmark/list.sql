SELECT
	b.id,
	b.name,
	b.url
FROM  bookmarkt.bookmark b
WHERE
	( {{ url }} IS NULL OR {{ type }} = b.url) AND
	( {{ name }} IS NULL OR b.name = {{ name }})
;
