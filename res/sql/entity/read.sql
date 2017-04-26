SELECT * FROM entity e WHERE (
   ({{ id }} IS NULL OR e.id = {{ id }}) AND
   ({{ uuid }} IS NULL OR e.uuid = {{ uuid }}) AND
   ({{ path }} IS NULL OR SUBSTR(e.path, 1, LENGTH( {{ path }} )) = {{ path }})
);
