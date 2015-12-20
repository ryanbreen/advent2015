var fs = require('fs');

var lines = fs.readFileSync("input.txt", 'utf8').split('\n');

var wires = {};

var Wire = function() {
  this.source = null;
  this.cached = null;

  this.evaluate = function() {
    console.log('Evaluating wire with source ' + this.source);

    if (this.cached) return this.cached; 

    if (!isNaN(this.source)) return parseInt(this.source);
    if (typeof this.source === 'object') {
      this.cached = this.source.evaluate();
      console.log("Value of wire is " + this.cached);
      return this.cached;
    }
    
    this.cached = wires[this.source].evaluate();
    console.log("Value of wire is " + this.cached);
    return this.cached;
  };
};

var NotGate = function(input, out) {
  out.source = this;
  this.input = input;

  this.evaluate = function() {
    return ~wires[this.input];
  };

  this.toString = function() {
    return "NOT " + this.input;
  };
};

var BooleanGate = function(is_and, left, right, out) {
  this.wire_l = left;
  this.wire_r = right;
  out.source = this;
  this.is_and = is_and;

  this.evaluate = function() {
    var left_val = isNaN(this.wire_l) ? wires[this.wire_l].evaluate() : parseInt(this.wire_l);
    var right_val = isNaN(this.wire_r) ? wires[this.wire_r].evaluate() : parseInt(this.wire_r);

    if (this.is_and) return left_val & right_val;
    else return left_val | right_val;
  };

  this.toString = function() {
    return (this.is_and ? "AND" : "OR") + " " + this.wire_l + " " + this.wire_r;
  };
};

var ShiftGate = function(is_left, input, shift_by, out) {
  this.input = input;
  this.shift_by = shift_by;
  out.source = this;
  this.is_left = is_left;

  this.evaluate = function() {
    var input = isNaN(this.input) ? wires[this.input].evaluate() : this.input;

    if (this.is_left) return input << this.shift_by;
    else return input >> this.shift_by;
  };

  this.toString = function() {
    return (this.is_left ? "LEFT" : "RIGHT") + " " + this.input + " " + this.shift_by;
  };
};

lines.forEach(function(line) {
  console.log(line);

  var parts = line.split(' ');
  if (parts.length === 1) return;

  if (line.indexOf('NOT') === 0) {
    var input_wire_label = parts[1];
    var output_wire = new Wire();
    wires[parts[3]] = output_wire;
    console.log(new NotGate(input_wire_label, output_wire));
  } else if (line.indexOf('AND') !== -1 || line.indexOf('OR') !== -1) {
    var left = parts[0];
    var right = parts[2];
    var output_wire = new Wire();
    wires[parts[4]] = output_wire;
    new BooleanGate(parts[1] === "AND", left, right, output_wire);
  } else if (line.indexOf('SHIFT') !== -1) {
    var left = parts[0];
    var right = parts[2];
    var output_wire = new Wire();
    wires[parts[4]] = output_wire;
    new ShiftGate(parts[1] === "LSHIFT", left, parseInt(right), output_wire);
  } else if (parts.length === 3) {
    var wire = new Wire();
    if (isNaN(parts[0])) wire.source = parts[0];
    else wire.source = parseInt(parts[0]);
    wires[parts[2]] = wire;
  } else {
    console.log("UNKNOWN TYPE " + line);
    process.exit(1);
  }

});

console.log(wires['a'].evaluate());