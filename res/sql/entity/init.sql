DROP TABLE entity;

CREATE TABLE IF NOT EXISTS entity (
    id TEXT NOT NULL PRIMARY KEY,
    parent_id TEXT NOT NULL,
    name TEXT NOT NULL,
    url TEXT,
    struct_type TEXT NOT NULL, -- Link, Node, Folder, Tag, Groupe...
    fn_type TEXT, -- Music, Playlist, Video, NULL
    rev_no INTEGER NOT NULL,

    FOREIGN KEY(parent_id) REFERENCES entity(id) ON DELETE CASCADE
);
