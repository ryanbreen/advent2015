var fs = require('fs');

var bytes = fs.readFileSync(process.argv[2], 'utf8');

var counter = 0;

for (var i=0; i<bytes.length; ++i) {
  if (bytes[i] == '(') counter += 1;
  if (bytes[i] == ')') counter -= 1;
}

console.log(counter);