var lines = require('fs').readFileSync(process.argv[2], 'utf8').split('\n');

var substitutions = {};
var target;

lines.forEach(function(line) {
  if (line.indexOf('=>') != -1) {
    var parts = line.split(' ');
    var subst = parts[0];
    if (!substitutions[subst]) substitutions[subst] = [];
    substitutions[subst].push(parts[2]);
  } else if (line.length > 0) {
    target = line;
  }
});

var subst_keys = Object.keys(substitutions);

console.log("Looking for %s in\n%s", target, subst_keys);

var uniques = {};

var recurse = function(current_str, remaining_string) {

  console.log('Recursing from %s %s', current_str, remaining_string)

  // if we have no remaining string, add this to the set of unique molecules
  if (remaining_string.length === 0) {
    if (current_str !== target) uniques[current_str] = true;
    return;
  }

  // if remaining string starts with a valid subst, recurse.
  subst_keys.forEach(function(subst) {
    if (remaining_string.startsWith(subst)) {
      var potentials = substitutions[subst];
      potentials.forEach(function(potential) {
        console.log('potential: %s', potential);
        uniques[current_str + potential + remaining_string.substring(subst.length)] = true;
      });
    }
  });

  // recurse without doing a subst
  recurse(current_str + remaining_string[0], remaining_string.substring(1));
};

recurse("", target);

console.log('Uniques: %s', Object.keys(uniques));
console.log('Count: %s', Object.keys(uniques).length);