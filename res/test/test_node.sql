CREATE TABLE IF NOT EXISTS link (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    node_id TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT NOT NULL,

    FOREIGN KEY(node_id) REFERENCES node(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node (
   id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
   name TEXT NOT NULL,
   node_id TEXT NOT NULL,

   FOREIGN KEY(node_id) REFERENCES node(node_id) ON DELETE CASCADE
);

INSERT INTO node (name, node_id) VALUES ('test_node', '1');
INSERT INTO node (name, node_id) VALUES ('test_node', '11');
INSERT INTO node (name, node_id) VALUES ('test_node', '12');
INSERT INTO node (name, node_id) VALUES ('test_node', '2');
INSERT INTO node (name, node_id) VALUES ('test_node', '21');
INSERT INTO node (name, node_id) VALUES ('test_node', '211');
INSERT INTO node (name, node_id) VALUES ('test_node', '22');
INSERT INTO node (name, node_id) VALUES ('test_node', '221');

INSERT INTO link (name, node_id, url) VALUE ('test_link1', '1');
INSERT INTO link (name, node_id, url) VALUE ('test_link2', '1');
INSERT INTO link (name, node_id, url) VALUE ('test_link3', '11');
INSERT INTO link (name, node_id, url) VALUE ('test_link4', '12');
INSERT INTO link (name, node_id, url) VALUE ('test_link5', '2');
INSERT INTO link (name, node_id, url) VALUE ('test_link6', '21');
INSERT INTO link (name, node_id, url) VALUE ('test_link7', '211');
INSERT INTO link (name, node_id, url) VALUE ('test_link8', '22');
INSERT INTO link (name, node_id, url) VALUE ('test_link9', '221');
