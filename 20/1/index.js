var _ = require('underscore');
var target = parseInt(process.argv[2]);

var process_house = function(num) {
  // Find all elves that would match this house
  var elves = [];

  for (var i = num; i > 0; --i) {
    if (num % i === 0) elves.push(i);
  }

  //console.log(num);
  //console.log(elves);

  return _.reduce(elves, function(memo, elf) { return memo + (elf * 10); }, 0);
};

/**/
var num = 289000;
var last_factor = 1.1;
var inc = true;

while (true) {
  num = Math.round(num);
  var val = process_house(num);

  console.log("House %d received %d gifts, within %s of target", num, val, target - val);

  if (val > target) return;

  if (inc) num++;
  else if (val > target) {
    num /= last_factor;
    last_factor -= .0001;
    console.log(last_factor)
    if (last_factor === 1) inc;
  } else if (last_factor > 1) {
    num *= last_factor;
  }

  // num++;
}

console.log(num);
/**/

//console.log(process_house(8));