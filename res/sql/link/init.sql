DROP TABLE link;

CREATE TABLE IF NOT EXISTS link (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    node_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    rev_no INTEGER NOT NULL,

    FOREIGN KEY(node_id) REFERENCES node(id)
);
