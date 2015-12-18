var fs = require('fs');

var bytes = fs.readFileSync(process.argv[2], 'utf8');

var touched = { }

var touch = function(x, y) {
  touched[x + 'x' + y] = true;
};

var current_x = 0;
var current_y = 0;

touch(current_x, current_y);

for (var i=0; i<bytes.length; ++i) {
  switch(bytes[i]) {
    case '^':
      current_y += 1;
      break;
    case 'v':
      current_y -= 1;
      break;
    case '>':
      current_x += 1;
      break;
    case '<':
      current_x -= 1;
      break;
  }

  touch(current_x, current_y);
}

console.log(Object.keys(touched).length);