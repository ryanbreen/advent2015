var last = 20151125;

var matrix = [
  [last]
];

var last_x = 0;
var last_y = 0;

//for (var i=2; i<22; ++i) {
while(true) {

  // console.log('%d %d', last_y, last_x);

  last *= 252533;
  last %= 33554393;

  // If we are as deep as the node to the left, back up
  if (last_y === 0) {
    // If we are in the first node of y, back all the way up.
    //console.log("Establishing new y at %d", last_y);
    last_x = 0;
    last_y = matrix[last_y].length;
    matrix[last_y] = [ last ];
  } else if (last_y !== 0 && matrix[last_y-1].length === matrix[last_y].length) {
    //console.log("Fill left")
    last_y -= 1;
    last_x = matrix[last_y].length;
    matrix[last_y][last_x] = last;
  } else if (matrix[last_y].length > matrix[last_y+1].length) {
    // if x.length > x+1.length, we need to fill in on the right
    //console.log("Fill right")
    last_y = last_y + 1;
    last_x = matrix[last_y].length;
    matrix[last_y][last_x] = last;
  }

  if (last_x === 3074 && last_y === 2980) {
    console.log("Value is %d", last);
    break;
  }
}

/**
for (var i=0; i<matrix.length; ++i) {
  console.log(matrix[i]);
}
**/