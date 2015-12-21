var fs = require('fs');

var obj = JSON.parse(fs.readFileSync("test.txt", 'utf8'));

var count = 0;

var recurse = function(obj) {

  console.log("looking at %s", obj);

  if (Array.isArray(obj)) {
    obj.forEach(function(entry) {
      recurse(entry);
    });
  } else if (typeof obj === 'object') {
    // Check for red before processing this object.
    var freak_out = false;
    Object.keys(obj).forEach(function(key) {
      if (obj[key] === 'red') freak_out = true;
    });
    if (freak_out) return;

    Object.keys(obj).forEach(function(key) {
      recurse(obj[key]);
    });
  } else if (typeof obj === 'number') {
    count += obj;
  }
};

recurse(obj);

console.log(count);