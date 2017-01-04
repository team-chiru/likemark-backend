DROP TABLE entity;

CREATE TABLE IF NOT EXISTS entity (
    id TEXT NOT NULL PRIMARY KEY,
    parent_id TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT,
    structure_type TEXT NOT NULL, -- Link, Node, Folder, Tag, Groupe...
    function_type TEXT, -- Music, Playlist, Video, NULL
    rev_no INTEGER NOT NULL,

    FOREIGN KEY(parent_id) REFERENCES entity(id) ON DELETE CASCADE
);

INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('1','1', 'test_node', "none", "folder", "music", 1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('2','11', 'test_node', "none", "folder","music", 1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('3','2', 'test_node', "none", "folder", "music", 1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('4','21', 'test_node', "none", "folder","music", 1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('5','211', 'test_node', "none", "folder","music",1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('6','22', 'test_node', "none", "folder","music", 1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('7','221', 'test_node',"none", "folder","music",1);

INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('8', '1',"link name" ,'www.test.com', "link","music",1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('9', '1',"link name" ,'www.test.com',"link","music",1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('10', '11', "link name",'www.test.com',"link","music",1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('11', '12',"link name",'www.test.com',"link","music",1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('12', '2', "link name",'www.test.com', "link","music",1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('13', '21',"link name",'www.test.com', "link","music",1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('14', '211',"link name",'www.test.com', "link","music",1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('15', '22',"link name",'www.test.com', "link","music",1);
INSERT INTO entity ( id, parent_id, name, url, structure_type, function_type, rev_no) VALUES ('15', '221',"link name",'www.test.com',"link","music",1 );
