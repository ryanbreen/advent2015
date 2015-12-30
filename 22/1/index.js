
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

  while (spells.length > 0) {
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

    console.log(spell)

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

function populate_arrays(parent, index) {
  for (var i=0; i<parent.length * parent.length; ++i) {
    for (var j=0; j<spells.length; ++j) {
      parent[i][index+j] = spells[j].name;
    }
  }
}

// Build an enumeration, from 1 to 20, of every combination.
for (var i=0; i<3; ++i) {
  // At each level, build an array of all spell combinations.
  combos[i] = [];
  var permutations = Math.pow(spells.length, i);
  for (var j=0; j<permutations; ++j) {
    combos[i][j] = [];

    for (var k=0; k<i; ++k) {
      console.log("Setting %d-%d-%d to spell %d", i, j, k, Math.floor(j / spells.length));
      combos[i][j][k] = spells[Math.floor(j / spells.length)].name;
    }
    
  }

}

console.log("There are %d possibilities", combos[1].length);
console.log(require('util').inspect(combos[1]));
console.log(require('util').inspect(combos[2].length));
console.log(require('util').inspect(combos[3].length));
/**
combos.sort(function(a, b) {
  return b.cost - a.cost;
});

console.log("Most expensive: %s", combos[0].cost);
console.log("Least expensive: %s", combos[combos.length-1].cost);

for (var i=0; i<combos.length; ++i) {

  // Reassert boss hp
  boss.hp = 103;

  if (simulate(combos[i], boss).name == "boss") {
    break;
  }
};

console.log("Most expensive loser is %s", require('util').inspect(combos[i]));
**/
