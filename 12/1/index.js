var fs = require('fs');

var obj = JSON.parse(fs.readFileSync("input.txt", 'utf8'));

var count = 0;

var recurse = function(obj) {

  console.log("looking at %s", obj);

  if (Array.isArray(obj)) {
    obj.forEach(function(entry) {
      recurse(entry);
    });
  } else if (typeof obj === 'object') {
    Object.keys(obj).forEach(function(key) {
      recurse(obj[key]);
    });
  } else if (typeof obj === 'number') {
    count += obj;
  }
};

recurse(obj);

console.log(count);