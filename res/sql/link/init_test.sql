 DELETE FROM link;
-- DELETE from sqlite_sequence WHERE name='bookmark';

INSERT INTO link
	(name, node_id, url, rev_no )
VALUES
	( "test", "test_url", 0 ),
	( "test", "test_url", 1 ),
	( "test", "test_url", 2 ),
	( "test", "test_url", 3 ),
	( "test", "test_url", 4 )
;
