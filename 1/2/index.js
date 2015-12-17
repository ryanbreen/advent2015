var fs = require('fs');

var bytes = fs.readFileSync(process.argv[2], 'utf8');

var counter = 0;

function find_basement_pos() {
  for (var i=0; i<bytes.length; ++i) {
    if (bytes[i] == '(') counter += 1;
    if (bytes[i] == ')') counter -= 1;

    if (counter < 0) return i + 1;
  }
}

console.log(find_basement_pos());