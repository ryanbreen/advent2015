var lines = require('fs').readFileSync(process.argv[2], 'utf8').split('\n');

var target = parseInt(process.argv[3]);

var boxes = [];

lines.forEach(function(line) {
  if (line.length > 0) boxes.push(parseInt(line));
});

console.log(boxes);

var matches = [];

var recurse = function(count, current_set, remainder) {

  if (remainder.length == 0) return;

  // Recurse down the no-add branch
  recurse(count, current_set, remainder.slice(1));

  var this_box = remainder[0];
  var new_set = current_set.slice(0);
  new_set.push(this_box);
  var new_count = count + this_box;
  if (new_count == target) return matches.push(new_set);
  else if (new_count > target) return;

  recurse(new_count, new_set, remainder.slice(1));
};

recurse(0, [], boxes);

matches.sort(function(a, b) {
  return b.length - a.length;
}).reverse();

var shortest = matches[0].length;

for (var i=0; i<matches.length && shortest === matches[i].length; ++i) {}

console.log("There are %s matches of length %s", i, shortest);