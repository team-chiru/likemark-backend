var parse = require("bookmarks-parser");
parse('<title>Pocket Export</title><h1>Unread</h1>'+
      '<ul><li><a href="http://example.com">Example!</a></li></ul>', function(err, res) {
  console.log(err);
  console.log(res.parser);
  console.log(res.bookmarks);
});
