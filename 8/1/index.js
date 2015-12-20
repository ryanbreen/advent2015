var fs = require('fs');

var bytes = fs.readFileSync('input.txt', 'utf8');

var total_byte_counter = 0;
var char_counter = 0;

for (var i=0; i<bytes.length; ++i) {

  // Skip newlines
  if (bytes[i] === '\n') continue;

  total_byte_counter += 1;

  if (bytes[i] != '"') char_counter += 1;

  // If this is the start of an escape sequence, decide how many bytes we need to skip
  if (bytes[i] === '\\') {
    if (bytes[i+1] === 'x') {
      // This is a hex code, so skip 4 bytes.
      total_byte_counter += 3;
      i += 3;
    } else {
      // Otherwise, this is a single escape.
      total_byte_counter += 1;
      i += 1;
    }
  }
}

console.log(total_byte_counter);
console.log(char_counter);
console.log(total_byte_counter - char_counter);