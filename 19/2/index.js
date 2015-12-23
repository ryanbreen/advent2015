var lines = require('fs').readFileSync(process.argv[2], 'utf8').split('\n');

var _ = require('underscore');

var substitutions = {};
var target = 'e';
var seed;

lines.forEach(function(line) {
  if (line.indexOf('=>') != -1) {
    var parts = line.split(' ');
    var subst = parts[2];
    if (!substitutions[subst]) substitutions[subst] = [];
    substitutions[subst].push(parts[0]);
  } else if (line.length > 0) {
    seed = line;
  }
});

var subst_keys = Object.keys(substitutions).sort(function(a, b) {
  return b.length - a.length;
});

console.log("Looking for %s from %s", target, subst_keys);

function recurse(seed, step) {
  if (seed === target) return step;

  console.log("Looking at %s as step %d", seed, step);

  for (var i=0; i<subst_keys.length; ++i) {
    
    var idx = seed.indexOf(subst_keys[i]);
    while (idx !== -1) {
      // Recurse once, replacing each instance.
      var replacements = substitutions[subst_keys[i]];
      for (var j=0; j<replacements.length; ++j) {
        var new_seed = seed.substring(0, idx) + replacements[j] + seed.substring(idx + subst_keys[i].length);
        var val = recurse(new_seed, step+1);
        if (val) return val;
      };

      idx = seed.indexOf(subst_keys[i], idx+1);
    }
  }
};

var result = recurse(seed, 0);
console.log(result);
