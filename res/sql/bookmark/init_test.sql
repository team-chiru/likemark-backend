DELETE FROM BOOKMARK ;

DELETE from sqlite_sequence
WHERE name='bookmark';

INSERT INTO bookmark
	(name, url, rev_no )
VALUES
	( "test", "test_url", 0 )
;
