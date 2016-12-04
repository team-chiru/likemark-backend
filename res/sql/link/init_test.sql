 DELETE FROM link;
-- DELETE from sqlite_sequence WHERE name='bookmark';

INSERT INTO link
	(name, node_id, url, rev_no )
VALUES
	("link", 1, "test_url", 0 ),
	("link", 2, "test_url", 0 ),
	("link", 2, "test_url", 0 ),
	("link", 3, "test_url", 0 ),
    ("link", 3, "test_url", 0 ),
	("link", 3, "test_url", 0 )
;
