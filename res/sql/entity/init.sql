-- DROP TABLE entity;

CREATE TABLE IF NOT EXISTS entity (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    tree_id TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT,
    struct_type TEXT NOT NULL, -- Link, Node, Folder, Tag, Groupe...
    fn_type TEXT, -- Music, Playlist, Video, NULL
    rev_no INTEGER NOT NULL,

    FOREIGN KEY(tree_id) REFERENCES entity(id) ON DELETE CASCADE,
    CONSTRAINT tree_id_unique UNIQUE (tree_id)
);
