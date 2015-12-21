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
      return b.priority - a.priority;
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

  this.longestWalk = function(start) {
    var visited = {};
    visited[start] = true;

    console.log('Starting with %s', start);

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
      console.log('Next is %s, link from %s was %s', test_vertex.key, current_label, test_vertex.priority);
      current_label = test_vertex.key;
      visited[test_vertex.key] = true;
    }

    // Close the loop by adding the link from the last to the first.
    total_cost += net_costs[start][current_label];

    return total_cost;
  }

}

var lines = require('fs').readFileSync('input.txt', 'utf8').split('\n');

var g = new Graph();
var net_costs = {};

lines.forEach(function(line) {
  var parts = line.split(' ');
  if (parts.length === 11) {
    var person_a = parts[0];
    var person_b = parts[10].substring(0, parts[10].length-1);

    var cost = parseInt(parts[3]);
    if (parts[2] === 'lose') cost = 0 - cost;

    console.log('Cost from %s to %s of cost %s', person_a, person_b, cost);
    if (!net_costs[person_a]) net_costs[person_a] = {};
    if (!net_costs[person_b]) net_costs[person_b] = {};

    if (!net_costs[person_a][person_b]) net_costs[person_a][person_b] = cost;
    else net_costs[person_a][person_b] = net_costs[person_a][person_b] + cost;

    if (!net_costs[person_b][person_a]) net_costs[person_b][person_a] = cost;
    else net_costs[person_b][person_a] = net_costs[person_b][person_a] + cost;
  }
});

Object.keys(net_costs).forEach(function(person_a) {
  Object.keys(net_costs[person_a]).forEach(function(person_b) {
    g.addVertex(person_a, person_b, net_costs[person_a][person_b]);
  });
});

var paths = [];
Object.keys(g.vertices).forEach(function(vertex) {
  paths.push(g.longestWalk(vertex));
});

console.log(net_costs);
console.log(paths);

console.log("Longest path: " + paths.sort().reverse()[0]);
