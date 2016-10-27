INSERT INTO bookmarkt.bookmark
	(id, name, url, time_created, stamp, revNo )
VALUES
	( {{ id }}, {{ name }}, {{ url }}, {{ stamp }}, {{ rev_no }} )
;
