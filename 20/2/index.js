var _ = require('underscore');
var target = parseInt(process.argv[2]);

var process_house = function(num) {
  // Find all elves that would match this house
  var elves = [];

  for (var i = (Math.floor(Math.sqrt(num))); i > 0; --i) {
    if (num % i === 0) {
      if (num <= i * 50) elves.push(i);
      var complement = num/i;
      if (complement != i && num <= complement * 50) elves.push(num/i);
    }
  }

  //console.log(elves);

  return _.reduce(elves, function(memo, elf) {
    // If this is more than the 50th house for this elf, skip.
    return memo + (elf * 11);
  }, 0);
};

var num = 0;
var val = 0;

while (val < target) {

  num += 1;
  val = process_house(num);
}

console.log(num);