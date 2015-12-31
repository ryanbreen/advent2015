var _ = require('underscore');
var util = require('util');

var spells = [{
  name: "Magic Missile",
  cost: 53,
  cast: function(effects, player, enemy) {
    enemy.hp -= 4;
  }
},{
  name: "Drain",
  cost: 73,
  cast: function(effects, player, enemy) {
    enemy.hp -= 2;
    player.hp += 2;
  }
},{
  name: "Shield",
  cost: 113,
  cast: function(effects, player, enemy) {
    var this_obj = this;
    var counter = 6;

    effects.push(function() {
      if (counter === 0) {
        delete player.active_spells[this_obj.name];
        return;
      }

      player.active_spells[this_obj.name] = true;

      player.armor += 7;

      counter--;
    });
  }
},{
  name: "Poison",
  cost: 173,
  cast: function(effects, player, enemy) {
    var this_obj = this;
    var counter = 6;

    effects.push(function() {
      if (counter === 0) {
        delete player.active_spells[this_obj.name];
        return;
      }

      player.active_spells[this_obj.name] = true;

      enemy.hp -= 3;

      counter--;
    });
  }
},{
  name: "Recharge",
  cost: 229,
  cast: function(effects, player, enemy) {
    var this_obj = this;
    var counter = 5;

    effects.push(function() {
      if (counter === 0) {
        delete player.active_spells[this_obj.name];
        return;
      }

      player.active_spells[this_obj.name] = true;

      player.mana += 101;

      counter--;
    });
  }
}];

var spell_lookup = {};

for (var i=0; i<spells.length; ++i) {
  spell_lookup[spells[i].name] = spells[i];
}

var test_boss = {
  name: "boss",
  hp: 14,
  damage: 8,
  armor: 0
};

var test = {
  name: "player",
  hp: 10,
  armor: 0,
  mana: 250,
  script: [
    'Poison',
    'Magic Missile'
  ]
};

var test2 = {
  name: "player",
  hp: 10,
  armor: 0,
  mana: 250,
  script: [
    'Recharge',
    'Shield',
    'Drain',
    'Poison',
    'Magic Missile'
  ]
};

var simulate = function(player, enemy) {
  var winner = null;
  var effects = [];

  player.active_spells = {};

  while (player.script.length > 0) {

    // Reset before running effects.
    player.armor = 0;

    //
    // Player turn
    //

    // Effects run first
    effects.forEach(function(effect) {
      effect();
    });

    // Now run the chosen spell
    var spell_name = player.script.shift();

    // Illegal move
    if (player.active_spells[spell_name]) {
      return enemy;
    }

    var spell = spell_lookup[spell_name];

    // Deduct cost
    player.mana -= spell.cost;
    if (player.mana < 0) return enemy;

    spell.cast(effects, player, enemy);

    if (player.hp <=0) return enemy;
    if (enemy.hp <= 0) return player;

    //console.log("After player turn, player is at %d (%d mana) and enemy is at %s", player.hp, player.mana, enemy.hp);

    //
    // Boss turn
    //

    // Reset before running effects.
    player.armor = 0;

    // Effects run first
    effects.forEach(function(effect) {
      effect();
    });

    if (enemy.hp <= 0) return player;

    //console.log("Enemy does %d damage", damage_dealt_by_enemy);
    var damage_dealt_by_enemy = Math.max(1, enemy.damage - player.armor);
    player.hp -= damage_dealt_by_enemy;
    if (player.hp <= 0) return enemy;

    //console.log("After boss turn, player is at %d (%d mana) and enemy is at %s", player.hp, player.mana, enemy.hp);
  }

  return;
};

// console.log(simulate(test2, test_boss).name);

var boss = {
  name: "boss",
  hp: 51,
  damage: 9
};

var combos = [];

/**
combos[1] = [
  [ "Magic Missile"],
  [ "Drain"],
  ...,
  [ "Recharge"]
]
combos[2] = [
  [ "Magic Missile", "Magic Missile"],
  [ "Magic Missile", "Drain"],
  ...,
  [ "Recharge", "Recharge"]
]
**/

function fill_out_permutations_at_level(level, max) {

  // Terminate condition
  if (level === max) {
    var rvalue = [];
    spells.forEach(function(spell) {
      rvalue.push([_.clone(spell)]);
    });
    return rvalue;
  }

  var lower = fill_out_permutations_at_level(level+1, max);

  // We now have an array of all combinations at the lower level.  Permute it at this level
  // and return the new array.
  var rvalue = [];
  lower.forEach(function(lower_permutation) {
    spells.forEach(function(spell) {
      // Clone the lower_permutation and unshift it.
      var new_permutation = _.clone(lower_permutation);
      new_permutation.unshift(_.clone(spell));

      rvalue.push(new_permutation);
    });
  });

  return rvalue;
}

function fill_out_permutations(max) {
  return fill_out_permutations_at_level(0, max);
}

//console.log(fill_out_permutations(1));

for (var i=0; i<10; ++i) {

  var permutations = fill_out_permutations(i);
  permutations.sort(function(a, b) {
    var reduce_fn = function(memo, spell) { 
      return memo + spell.cost;
    };
    return _.reduce(a, reduce_fn, 0) - _.reduce(b, reduce_fn, 0);
  });

  permutations.forEach(function(permutation) {
    var script = _.pluck(permutation, 'name');
    var total_cost = _.reduce(_.pluck(permutation, 'cost'), function(memo, num) { return memo + num });
    console.log("Testing with script of cost", total_cost);
    var player = {
      hp: 50,
      mana: 500,
      script: script
    };
    var result = simulate(player, boss);
    if (result == player) {
      console.log("Winning player is %s", util.inspect(player));
      process.exit(0);
    } else if (result == boss) {
      console.log("Boss won");
    } else {
      console.log("Inconclusive");
    }

    boss.hp = 51;
  });

}
