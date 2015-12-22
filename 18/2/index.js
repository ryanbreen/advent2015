
var Node = function(is_on) {

  this.is_on = is_on;
  this.neighbors = [];

  this.tick = function() {
    // Look at the current state of the neighbors and determine what our new state should be.
    
  };
}

var lines = require('fs').readFileSync(process.argv[2], 'utf8').split('\n');

var row_count = 0;
var current_grid = [];
lines.forEach(function(line) {
  if (line.length < 1) return;

  current_grid[row_count] = [];

  for (var i=0; i<line.length; ++i) {
    if (line[i] == '#') current_grid[row_count].push(true)
    else current_grid[row_count].push(false);
  }

  row_count++;
});

var grids = [ current_grid ];

function safe_grab_cell(neighbors, grid, i, j) {
  if (i<0 || i>=row_count) return;
  if (j<0 || j>=row_count) return;

  neighbors.push(grid[i][j]);
}

function grab_adjacent_on_count(grid, i, j) {
  var on_count = 0;

  var neighbors = [];

  // The normal bounds are i-1 to i+1 and j-1 to j+1
  safe_grab_cell(neighbors, grid, i-1, j);
  safe_grab_cell(neighbors, grid, i-1, j-1);
  safe_grab_cell(neighbors, grid, i-1, j+1);
  safe_grab_cell(neighbors, grid, i, j+1);
  safe_grab_cell(neighbors, grid, i, j-1);
  safe_grab_cell(neighbors, grid, i+1, j-1);
  safe_grab_cell(neighbors, grid, i+1, j);
  safe_grab_cell(neighbors, grid, i+1, j+1);

  neighbors.forEach(function(on) {
    if (on) on_count += 1;
  });

  return on_count;
};

for (var c=0; c<process.argv[3]; ++c) {

  var new_grid = [];

  for (var i=0; i<current_grid.length; ++i) {
    var row = current_grid[i];

    new_grid[i] = [];

    for (var j=0; j<row.length; ++j) {

      // short circuit for the corner case
      if ((i === 0 && j === 0) ||
        (i === 0 && j === row.length-1) || 
        (i === row.length-1 && j === row.length-1) || 
        (i === row.length-1 && j === 0)) {
        new_grid[i][j] = true;
        continue;
      }

      var is_on = current_grid[i][j];

      // Find adjacent neighbors
      var on_neighbor_count = grab_adjacent_on_count(current_grid, i, j);
      if (is_on) {
        if (on_neighbor_count === 2 || on_neighbor_count === 3) new_grid[i][j] = true;
        else new_grid[i][j] = false;
      } else {
        if (on_neighbor_count === 3) new_grid[i][j] = true;
        else new_grid[i][j] = false;
      }
    }
  }

  grids.push(new_grid);

  current_grid = new_grid;
}

var count = 0;
current_grid.forEach(function(row) {
  row.forEach(function(cell) {
    if (cell) ++count;
  });
});

console.log('%s lights are lit', count);