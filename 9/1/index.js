var util = require('util');

function PriorityQueue () {
  this._nodes = [];

  this.enqueue = function (priority, key) {
    this._nodes.push({key: key, priority: priority });
    this.sort();
  }
  this.dequeue = function () {
    return this._nodes.shift();
  }
  this.sort = function () {
    this._nodes.sort(function (a, b) {
      return a.priority - b.priority;
    });
  }
  this.isEmpty = function () {
    return !this._nodes.length;
  }
}

/**
 * Pathfinding starts here
 */
function Graph(){
  var INFINITY = 1/0;
  this.vertices = {};

  this.addVertex = function(name, edge_name, edge_cost){
    if (!this.vertices[name]) this.vertices[name] = new PriorityQueue();

    this.vertices[name].enqueue(edge_cost, edge_name);
  }

  this.shortestWalk = function(start) {
    var visited = {};
    visited[start] = true;

    var current_label = start;
    var total_cost = 0;
    while (Object.keys(visited).length !== Object.keys(this.vertices).length) {

      var i = 0;
      current_vertex = this.vertices[current_label];
      var test_vertex = current_vertex._nodes[i];
      while (visited[test_vertex.key]) {
        ++i;
        test_vertex = current_vertex._nodes[i];
      }

      total_cost += test_vertex.priority;
      current_label = test_vertex.key;
      visited[test_vertex.key] = true;
    }

    return total_cost;
  }

}

var lines = require('fs').readFileSync('input.txt', 'utf8').split('\n');

var g = new Graph();

lines.forEach(function(line) {
  var parts = line.split(' ');
  if (parts.length === 5) {
    console.log('Adding vertex from %s to %s of cost %s', parts[0], parts[2], parts[4])
    g.addVertex(parts[0], parts[2], parseInt(parts[4]));
    g.addVertex(parts[2], parts[0], parseInt(parts[4]));
  }
});

var paths = [];
Object.keys(g.vertices).forEach(function(vertex) {
  paths.push(g.shortestWalk(vertex));
});

console.log("Shortest path: " + paths.sort()[0]);
