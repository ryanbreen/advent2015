
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
    var counter = 6;

    effects.push(function() {
      if (counter === 0) return;

      player.armor += 7;

      counter--;
    });
  }
},{
  name: "Poison",
  cost: 173,
  cast: function(effects, player, enemy) {
    var counter = 6;

    effects.push(function() {
      if (counter === 0) return;

      enemy.hp -= 3;

      counter--;
    });
  }
},{
  name: "Recharge",
  cost: 229,
  cast: function(effects, player, enemy) {
    var counter = 5;

    effects.push(function() {
      if (counter === 0) return;

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
    var spell = spell_lookup[spell_name];
    console.log(spell)

    // Deduct cost
    player.mana -= spell.cost;
    if (player.mana < 0) return enemy;

    spell.cast(effects, player, enemy);

    if (player.hp <=0) return enemy;
    if (enemy.hp <= 0) return player;

    console.log("After player turn, player is at %d (%d mana) and enemy is at %s", player.hp, player.mana, enemy.hp);

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

    console.log("After boss turn, player is at %d (%d mana) and enemy is at %s", player.hp, player.mana, enemy.hp);
  };
};

console.log(simulate(test2, test_boss).name);

/**
var boss = {
  name: "boss",
  hp: 103,
  damage: 9,
  armor: 2
};

var combos = [];

// Build an enumeration of all possible armor, ring, and weapon combos
for (var i=-1; i<armor.length; ++i) {
  // You must buy a weapon, so no skipping.
  for (var j=0; j<weapons.length; ++j) {
    for (var k=-1; k<rings1.length; ++k) {
      for (var l=-1; l<rings2.length; ++l) {
        var combo = {
          hp: 100,
          armor: 0,
          damage: 0,
          cost: 0
        };

        if (i != -1) {
          combo.armor += armor[i].armor;
          combo.cost += armor[i].cost;
        }

        combo.damage += weapons[j].damage;
        combo.cost += weapons[j].cost;

        if (k != -1) {
          combo.armor += rings1[k].armor;
          combo.damage += rings1[k].damage;
          combo.cost += rings1[k].cost;
        }

        if (l != -1 && l !== k) {
          combo.armor += rings2[l].armor;
          combo.damage += rings2[l].damage;
          combo.cost += rings2[l].cost;
        }

        combos.push(combo);
      }
    }
  }
}

console.log("There are %d possibilities", combos.length);
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
