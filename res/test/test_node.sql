CREATE TABLE IF NOT EXISTS link (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    node_id TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT NOT NULL,

    FOREIGN KEY(node_id) REFERENCES node(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node (
   id TEXT NOT NULL,
   name TEXT NOT NULL,
   node_id TEXT NOT NULL,

   FOREIGN KEY(node_id) REFERENCES node(node_id) ON DELETE CASCADE
);

INSERT INTO node (id, name, node_id) VALUES ('1', 'test_node', '0');
INSERT INTO node (id, name, node_id) VALUES ('11', 'test_node', '1');
INSERT INTO node (id, name, node_id) VALUES ('12', 'test_node', '1');
INSERT INTO node (id, name, node_id) VALUES ('2', 'test_node', '0');
INSERT INTO node (id, name, node_id) VALUES ('21', 'test_node', '2');
INSERT INTO node (id, name, node_id) VALUES ('211', 'test_node', '21');
INSERT INTO node (id, name, node_id) VALUES ('22', 'test_node', '2');
INSERT INTO node (id, name, node_id) VALUES ('221', 'test_node', '22');

INSERT INTO link (name, node_id, url) VALUES ('test_link1', '1', 'www.test.com');
INSERT INTO link (name, node_id, url) VALUES ('test_link2', '1', 'www.test.com');
INSERT INTO link (name, node_id, url) VALUES ('test_link3', '11','www.test.com');
INSERT INTO link (name, node_id, url) VALUES ('test_link4', '12', 'www.test.com');
INSERT INTO link (name, node_id, url) VALUES ('test_link5', '2', 'www.test.com');
INSERT INTO link (name, node_id, url) VALUES ('test_link6', '21', 'www.test.com');
INSERT INTO link (name, node_id, url) VALUES ('test_link7', '211', 'www.test.com');
INSERT INTO link (name, node_id, url) VALUES ('test_link8', '22', 'www.test.com');
INSERT INTO link (name, node_id, url) VALUES ('test_link9', '221', 'www.test.com');

SELECT l.name, n.id, n.node_id FROM link l
LEFT JOIN node n ON l.node_id = n.id
WHERE
	SUBSTR(n.id, 1, LENGTH('21')) = '21';

/* FINAL QUERY:
SELECT l.name, n.id, n.node_id FROM link l
LEFT JOIN node n ON l.node_id = n.id
WHERE
	SUBSTR(n.id, 1, LENGTH({{ node_id }})) = {{ node_id }};
*/

/*
NOTES/SPECS:
tout les ids marchent, on peut faire varier la taille des id générés
(exemple: 1b3& ce qui donne 4*[0-9a-zA-Z!@#$%&^*...] possibilités)
CONDITION HYPER MEGA IMPORTANTE:
chaque id d'un niveau doit contenir l'id parent!
*/
