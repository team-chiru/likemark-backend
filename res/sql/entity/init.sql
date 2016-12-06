DROP TABLE entity;

CREATE TABLE IF NOT EXISTS entity (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    parent_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    url TEXT,
    structure_type TEXT NOT NULL, -- Link, Node, Folder, Tag, Groupe...
    function_type TEXT, -- Music, Playlist, Video, NULL
    rev_no INTEGER NOT NULL,

    FOREIGN KEY(parent_id) REFERENCES node(id) ON DELETE CASCADE
);
