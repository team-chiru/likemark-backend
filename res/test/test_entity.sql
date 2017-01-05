DROP TABLE entity;

CREATE TABLE IF NOT EXISTS entity (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    parent_id TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT,
    struct_type TEXT NOT NULL, -- Link, Node, Folder, Tag, Groupe...
    fn_type TEXT, -- Music, Playlist, Video, NULL
    rev_no INTEGER NOT NULL,

    FOREIGN KEY(parent_id) REFERENCES entity(id) ON DELETE CASCADE,
    CONSTRAINT parent_id_unique UNIQUE (parent_id)
);

INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('1', 'test_node', "none", "folder", "music", 1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('11', 'test_node', "none", "folder","music", 1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('2', 'test_node', "none", "folder", "music", 1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('21', 'test_node', "none", "folder","music", 1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('211', 'test_node', "none", "folder","music",1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('22', 'test_node', "none", "folder","music", 1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('221', 'test_node',"none", "folder","music",1);

INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('1',"link name" ,'www.test.com', "link","music",1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('1',"link name" ,'www.test.com',"link","music",1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('11', "link name",'www.test.com',"link","music",1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('12',"link name",'www.test.com',"link","music",1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('2', "link name",'www.test.com', "link","music",1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('21',"link name",'www.test.com', "link","music",1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('211',"link name",'www.test.com', "link","music",1);
INSERT INTO entity (parent_id, name, url, struct_type, fn_type, rev_no) VALUES ('22',"link name",'www.test.com', "link","music",1);

-- SELECT * FROM entity e
-- WHERE
-- 	SUBSTR(e.parent_id, 1, LENGTH( {{ parent_id }} )) = {{ parent_id }};
