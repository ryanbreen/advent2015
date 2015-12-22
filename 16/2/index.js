var lines = require('fs').readFileSync(process.argv[2], 'utf8').split('\n');

var ingredients = {};

var criteria = {
  children: 3,
  cats: function (input) {
    return input > 7;
  },
  samoyeds: 2,
  pomeranians: function (input) {
    return input < 3;
  },
  akitas: 0,
  vizslas: 0,
  goldfish: function (input) {
    return input < 5;
  },
  trees: function (input) {
    return input > 3;
  },
  cars: 2,
  perfumes: 1
};

lines.forEach(function(line) {
  var parts = line.split(' ');
  if (parts.length > 4) {
    var id = parts[1].substring(0, parts[1].length-1);
    var matched = true;
    for (var i=2; i<parts.length; i+=2) {
      if (parts[i+1].indexOf(',') !== -1) parts[i+1] = parts[i+1].substring(0, parts[i+1].length-1);
      var type = parts[i].substring(0, parts[i].length-1);
      var val = parseInt(parts[i+1]);
      if (typeof criteria[type] == 'function') {
        if (!criteria[type](val)) return;
      } else {
        if (criteria[type] !== val) return;
      }
    }

    console.log("Found match %s", id);
  }
});

