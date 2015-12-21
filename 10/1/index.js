var seed = process.argv[2]

function munge_me(input) {

  var output = "";
  for (var i=0; i<input.length; ) {
    var current_char = input[i];
    var current_char_str = current_char;

    var j = 1;
    // Look ahead until we hit the end of the string or another char.
    while (i+j < input.length && input[i+j] == current_char) {
      current_char_str += current_char;
      j += 1;
    }

    output += (current_char_str.length + current_char);

    i+=j;
  }

  return output;
}

var wamp = seed;

for (var i=0; i<40; ++i) {
  wamp = munge_me(wamp);
}

console.log("%s=>%s", seed, wamp);
console.log("length=>%s", wamp.length);