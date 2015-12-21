
var Deer = function(name, speed, pause_after, pause_duration) {
  this.name = name;
  this.speed = speed;
  this.pause_after = pause_after;
  this.pause_duration = pause_duration;

  this.current_pos = 0;
  this.is_paused = false;
  this.paused_for = 0;

  this.running_for = 0;

  this.tick = function() {
    if (this.is_paused) {
      this.paused_for += 1;

      if (this.paused_for >= this.pause_duration) {
        this.paused_for = 0;
        this.is_paused = false;
      }
    } else {
      this.running_for += 1;

      this.current_pos += speed;

      if (this.running_for >= this.pause_after) {
        this.running_for = 0;
        this.is_paused = true;
      }
    }
  };
}

var simulation = {

  deer: [],

  run: function(duration) {
    for (var current_time = 0; current_time < duration; current_time += 1) {
      for (var i=0; i<simulation.deer.length; ++i) {
        simulation.deer[i].tick();
      }
    }
  }

};

var lines = require('fs').readFileSync(process.argv[2], 'utf8').split('\n');

lines.forEach(function(line) {
  var parts = line.split(' ');
  if (parts.length === 15) {
    simulation.deer.push(new Deer(parts[0], parseInt(parts[3]), parseInt(parts[6]), parseInt(parts[13])));
  }
});

simulation.run(process.argv[3]);

simulation.deer.sort(function(a, b) {
  return b.current_pos - a.current_pos;
});

console.log(simulation.deer);