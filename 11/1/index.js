var seed = process.argv[2]

// 97-122 excluding 105, 108, and 111

function increment_password(input) {

  var wrapping = false;

  var output_codes = [];

  var incrementing = true;

  for (var i=input.length-1; i>=0; --i) {

    var current = input.charCodeAt(i);
    if (incrementing) {
      current += 1;
      incrementing = false;

      if (current === 105 || current === 108 || current === 111) {
        current += 1;
      } else if (current === 123) {
        current = 97;
        incrementing = true;
      }
    }

    output_codes[i] = current;

  }

  var output = '';
  for (var i=0; i<output_codes.length; ++i) {
    output += String.fromCharCode(output_codes[i]);
  }

  return output;
}

function is_password_valid(input) {

  // Check for invalid chars
  for (var i=0; i<input.length-1; ++i) {
    var current = input.charCodeAt(i);
    if (current === 105 || current === 108 || current === 111) return false;
  }

  // Check for incrementing run
  var inc_met = false;
  for (var i=0; i<input.length-2; ++i) {
    if ((input.charCodeAt(i+2) - input.charCodeAt(i+1) == 1) &&
      (input.charCodeAt(i+1) - input.charCodeAt(i) == 1)) {
      inc_met = true;
      break;
    }
  }

  if (!inc_met) return false;

  var first_matched = null;
  for (var i=0; i<input.length-1; ++i) {
    if (input[i] === first_matched) continue;

    if (input[i+1] === input[i]) {
      // We've found a pair.
      if (first_matched !== null) return true;
      first_matched = input[i];
    }
  }

  return false;
}

var pass = seed;
do {
  pass = increment_password(pass);
} while(!is_password_valid(pass));
console.log(pass);