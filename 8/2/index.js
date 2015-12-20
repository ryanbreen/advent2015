var fs = require('fs');

var lines = fs.readFileSync('input.txt', 'utf8').split('\n');

var total_byte_counter = 0;
var char_counter = 0;

lines.forEach(function(line) {
  if (line === '') return;

  var str = '"';

  for (var i=0; i<line.length; ++i) {
    var char = line.charAt(i);

    if (char === '\\' || char === '"') str += '\\';

    str += char;
  }

  str += '"';

  console.log(line + "(" + line.length + ")==>" + str + "(" + str.length + ")");

  total_byte_counter += line.length;
  char_counter += str.length;
});

console.log(total_byte_counter);
console.log(char_counter);
console.log(char_counter - total_byte_counter);
