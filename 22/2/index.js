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
    var disabled = false;

    effects.push(function() {
      if (disabled) return;

      player.active_spells[this_obj.name] = true;

      player.armor += 7;

      counter--;
      if (counter === 0) {
        delete player.active_spells[this_obj.name];
        disabled = true;
        return;
      }
    });
  }
},{
  name: "Poison",
  cost: 173,
  cast: function(effects, player, enemy) {
    var this_obj = this;
    var counter = 6;
    var disabled = false;

    effects.push(function() {
      if (disabled) return;

      player.active_spells[this_obj.name] = true;

      enemy.hp -= 3;

      counter--;
      if (counter === 0) {
        delete player.active_spells[this_obj.name];
        disabled = true;
        return;
      }
    });
  }
},{
  name: "Recharge",
  cost: 229,
  cast: function(effects, player, enemy) {
    var this_obj = this;
    var counter = 5;
    var disabled = false;

    effects.push(function() {
      if (disabled) return;

      player.active_spells[this_obj.name] = true;

      player.mana += 101;

      counter--;
      if (counter === 0) {
        delete player.active_spells[this_obj.name];
        disabled = true;
        return;
      }
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

    // HARD MODE ENGAGED
    player.hp -= 1;
    if (player.hp <=0) return enemy;

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

var cheapest_winner;

function test_permutations_at_level(level, max, permutation) {

  // Terminate condition
  if (level === max) {
    spells.forEach(function(spell) {
      permutation[level] = spell;

      // test
      var script = _.pluck(permutation, 'name');
      var total_cost = _.reduce(_.pluck(permutation, 'cost'), function(memo, num) { return memo + num });
      var player = {
        hp: 50,
        mana: 500,
        script: script
      };
      var result = simulate(player, boss);
      if (result == player) {
        // console.log("Winning player is %s", util.inspect(player));
        if (!cheapest_winner) cheapest_winner = total_cost;
        else if (total_cost < cheapest_winner) cheapest_winner = total_cost;
        console.log("Cheapest winner is currently %d", cheapest_winner);
      } else if (result == boss) {
        //console.log("Boss won");
      } else {
        //console.log("Inconclusive");
      }

      boss.hp = 51;
    });
  } else {
    spells.forEach(function(spell) {
      permutation[level] = spell;
      test_permutations_at_level(level+1, max, permutation);
    });
  }
}

function test_permutations(max) {
  return test_permutations_at_level(0, max, []);
}

//console.log(fill_out_permutations(1));

for (var i=0; i<15; ++i) {

  console.log("Testing permutations of depth %d.", i);
  test_permutations(i);
}
