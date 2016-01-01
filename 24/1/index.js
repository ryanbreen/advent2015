var _ = require('underscore');
var util = require('util');

var test_input = [
  1, 2, 3, 4, 5, 7, 8, 9, 10, 11
];

var prod_input = [1,3,5,11,13,17,19,23,29,31,37,41,43,47,53,59,67,71,73,79,83,89,97,101,103,107,109,113]

//var input = test_input;
var input = prod_input;

input.sort(function(a, b) {
  return a-b
});

var waterline = _.reduce(input, function(memo, num) { return memo + num; }, 0) / 3;
console.log("Looking for thirds of size %d", waterline);

var valid_slices = [];
var waterline_exceeded = false;
var test_values = [];

var traverse = function(i, len) {
  for (var j=0; j<input.length && !waterline_exceeded; ++j) {
    // Skip cases where we have already selected this index
    if (_.contains(test_values, input[j])) {
      //console.log("Skipping because %s contains %d", test_values, input[j]);
      continue;
    };

    test_values[i] = input[j];

    // If we have filled the test array, test.
    if (i === len - 1) {
      //console.log(test_values);

      test_values[i] = input[j];
      var sum = _.reduce(test_values, function(memo, num) { return memo + num; }, 0);
      // console.log(sum);
      if (sum === waterline) valid_slices.push(_.clone(test_values));
      //else if (sum > waterline) waterline_exceeded = true;
    } else {
      traverse(i+1, len);
    }
  }
}

// Generate all possible slices of this length and test them.
function test_slices_of_length(len) {

  if (len > input.length) {
    console.error("Invalid length %d", len);
    process.exit(2);
  }

  // Reset waterline exceeeded
  waterline_exceeded = false;

  console.log("Testing slices of length %d", len);
  traverse(0, len);

  if (valid_slices.length > 0) {
    // Which one has the lowest QE?
    valid_slices.sort(function(a, b) {
      var a_product = _.reduce(a, function(memo, num) { return memo * num; }, 1);
      var b_product = _.reduce(b, function(memo, num) { return memo * num; }, 1);
      return a_product - b_product;
    });

    console.log("Found valid pairs of length %d", len);
    console.log("Valid pairs:\n%s", util.inspect(valid_slices));

    return true;
  }

  return false;
}

for (var i=1; !test_slices_of_length(i); ++i) {}

var product_fn = function(memo, num) { return memo * num; };

valid_slices.sort(function(a, b) {
  return _.reduce(a, product_fn, 1) - _.reduce(b, product_fn, 1);
});

console.log("QE of lowest is %d", _.reduce(valid_slices[0], product_fn, 1));