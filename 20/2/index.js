var _ = require('underscore');
var cluster = require('cluster');
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

var num = 5040000;

var numCPUs = require('os').cpus().length;
if (cluster.isMaster) {
  // Fork workers.
  for (var i = 0; i < numCPUs; i++) {
    cluster.fork({NUM: num - i});
  }

  cluster.on('exit', function(worker, code, signal) {
    console.log('worker ' + worker.process.pid + ' died');
  });
} else {

  /**/
  var val = 0;
  var num = process.env['NUM'];

  while (val < target) {

    val = process_house(num);

    if (num % 10000 === 0) console.log("House %d received %d gifts, within %s of target", num, val, target - val);

    num -= numCPUs;
  }

  console.log(num);
}
