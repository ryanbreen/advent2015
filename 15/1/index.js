var lines = require('fs').readFileSync(process.argv[2], 'utf8').split('\n');

var ingredients = {};

lines.forEach(function(line) {
  var parts = line.split(' ');
  if (parts.length === 11) {
    ingredients[parts[0].substring(0, parts[0].length-1)] = {
      capacity: parseInt(parts[2].substring(0, parts[2].length-1)),
      durability: parseInt(parts[4].substring(0, parts[4].length-1)),
      flavor: parseInt(parts[6].substring(0, parts[6].length-1)),
      texture: parseInt(parts[8].substring(0, parts[8].length-1)),
      calories: parseInt(parts[10])
    }
  }
});

console.log(ingredients);

var ingredient_names = Object.keys(ingredients);

var current_max = 0;
var current_max_allocation = [];

var current_allocation = [];

var recurse = function(position, remainder, remaining_ingredients) {

  for (var i=0; i<remainder; ++i) {
    current_allocation[position] = remainder - i;

    if (remaining_ingredients.length === 0) {

      // If we are at the last ingredient, calculate the sum and see how much we have.
      var capacity = 0;
      var durability = 0;
      var flavor = 0;
      var texture = 0;
      for (var j=0; j<current_allocation.length; ++j) {
        capacity += ingredients[ingredient_names[j]].capacity * current_allocation[j];
        durability += ingredients[ingredient_names[j]].durability * current_allocation[j];
        flavor += ingredients[ingredient_names[j]].flavor * current_allocation[j];
        texture += ingredients[ingredient_names[j]].texture * current_allocation[j];
      }

      var sum = Math.max(capacity, 0) * Math.max(durability, 0) * Math.max(flavor, 0) * Math.max(texture, 0);
      if (sum > current_max) {
        current_max = sum;
        current_max_allocation = current_allocation.slice(0);
      }

    } else {
      // Keep recursing in the free world.
      recurse(position+1, i, remaining_ingredients.slice(1));
    }
  }

};

recurse(0, 100, ingredient_names.slice(1));

console.log(current_max_allocation);
console.log(current_max);