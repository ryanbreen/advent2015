var _ = require('underscore');
var target = parseInt(process.argv[2]);

var process_house = function(num) {
  // Find all elves that would match this house
  var elves = [];

  for (var i = (num/2)+1; i > 0; --i) {
    if (num % i === 0) elves.push(i);
  }

  //console.log(num);
  //console.log(elves);

  return _.reduce(elves, function(memo, elf) {
    // If this is more than the 50th house for this elf, skip.
    if (elf * 50 > num) return memo;

    return memo + (elf * 10);
  }, 0);
};

/**/
var num = 0;
var val = 0;

while (val < target) {

  if ((num.toString().match(/0/g) || []).length < 3) {
    ++num;
    continue;
  }

  val = process_house(num);

  console.log("House %d received %d gifts, within %s of target", num, val, target - val);

  ++num;
}

console.log(num);
/**/

//console.log(process_house(8));