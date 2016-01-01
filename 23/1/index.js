var tape = [];

var pc = 0;

var registers = {
  a: 0,
  b: 0
};

var instructions = {

  // hlf r sets register r to half its current value, then continues with the next instruction.
  hlf : function(register) {
    return function() {
      registers[register] = registers[register] / 2;
      console.log("Halved register %s to %d", register, registers[register]);
      pc += 1;
    };
  },

  // tpl r sets register r to triple its current value, then continues with the next instruction.
  tpl : function(register) {
    return function() {
      registers[register] = registers[register] * 3;
      console.log("Tripled register %s to %d", register, registers[register]);
      pc += 1;
    };
  },

  // inc r increments register r, adding 1 to it, then continues with the next instruction.
  inc : function(register) {
    return function() {
      console.log("Increment register %s", register);
      registers[register] += 1;
      pc += 1;
    };
  },

  // jmp offset is a jump; it continues with the instruction offset away relative to itself.
  jmp : function(distance) {
    return function() {
      console.log("Jump %d", distance);
      pc += distance;
      console.log("Set pc to %d", pc);
    };
  },

  // jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
  jie : function(register, distance) {
    return function() {
      if (registers[register] % 2 === 0) {
        console.log("Jumping by %d because register %s is even", distance, register);
        pc += distance;
      } else pc += 1;
    };
  },

  // jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
  jio : function(register, distance) {
    return function() {
      if (registers[register] === 1) {
        console.log("Jumping by %d because register %s is 1", distance, register);
        pc += distance;
      } else pc += 1;
    };
  }

}

var code = require('fs').readFileSync(process.argv[2], 'utf8').split('\n');
code.forEach(function(line) {
  if (line === "") return;
  var parts = line.split(' ');
  var instruction_fn = instructions[parts[0]];
  if (parts[0] === 'jie' || parts[0] === 'jio') {
    tape.push(instruction_fn(parts[1].substring(0, parts[1].length-1), parseInt(parts[2])));
  } else if (parts[0] == 'jmp') {
    tape.push(instruction_fn(parseInt(parts[1])));
  } else {
    tape.push(instruction_fn(parts[1]));
  }
});

//console.log("Tape is:\n%s", require('util').inspect(tape));

console.log("Tape length is %d", tape.length);

while (pc < tape.length) {
  console.log(registers);
  console.log(pc);
  console.log(code[pc]);
  tape[pc]();
}

console.log(registers);