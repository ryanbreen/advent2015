
var md5 = require('md5');

function mine(seed) {

  var pad = 1;
  while (true) {
    var hash = md5(seed + pad);
    if (hash.indexOf('000000') === 0) return pad;
    pad += 1;
  }
}

console.log(mine(process.argv[2]))