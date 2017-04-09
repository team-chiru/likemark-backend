/* need to update all children TOO! */
UPDATE entity SET
    path = {{ path }},
    name = {{ name }},
    url = {{ url }},
    struct_type = {{ struct_type }},
    fn_type = {{ fn_type }},
    rev_no = rev_no + 1
WHERE (
   ({{ id }} IS NULL OR id = {{ id }}) AND
   ({{ uuid }} IS NULL OR uuid = {{ uuid }})
);
