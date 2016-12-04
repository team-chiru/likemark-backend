DELETE FROM node;
-- DELETE from sqlite_sequence WHERE name='bookmark';

INSERT INTO node
    (name, type, node_id, rev_no)
VALUES
    ("test_node", "folder", 1, 0),
    ("test_node", "folder", 2, 0),
    ("test_node", "folder", 2, 0),
    ("test_node", "folder", 3, 0),
    ("test_node", "folder", 3, 0),
    ("test_node", "folder", 3, 0)
;
