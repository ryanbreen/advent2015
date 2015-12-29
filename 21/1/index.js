
var weapons = [{
  name: "Dagger",
  cost: 8,
  damage: 4,
  armor: 0
},{
  name: "Shortsword",
  cost: 10,
  damage: 5,
  armor: 0
},{
  name: "Warhammer",
  cost: 25,
  damage: 6,
  armor: 0
},{
  name: "Longsword",
  cost: 40,
  damage: 7,
  armor: 0
},{
  name: "Greataxe",
  cost: 74,
  damage: 8,
  armor: 0
}];

var armor = [{
  name: "Leather",
  cost: 13,
  damage: 0,
  armor: 1
},{
  name: "Chainmail",
  cost: 31,
  damage: 0,
  armor: 2
},{
  name: "Splintmail",
  cost: 53,
  damage: 0,
  armor: 3
},{
  name: "Bandedmail",
  cost: 75,
  damage: 0,
  armor: 4
},{
  name: "Platemail",
  cost: 102,
  damage: 0,
  armor: 5
}];

var rings1 = [{
  name: "Damage +1",
  cost: 25,
  damage: 1,
  armor: 0
},{
  name: "Damage +2",
  cost: 50,
  damage: 2,
  armor: 0
},{
  name: "Damage +3",
  cost: 100,
  damage: 3,
  armor: 0
},{
  name: "Defense +1",
  cost: 20,
  damage: 0,
  armor: 1
},{
  name: "Defense +2",
  cost: 40,
  damage: 0,
  armor: 2
},{
  name: "Defense +3",
  cost: 80,
  damage: 0,
  armor: 3
}];

var rings2 = [{
  name: "Damage +1",
  cost: 25,
  damage: 1,
  armor: 0
},{
  name: "Damage +2",
  cost: 50,
  damage: 2,
  armor: 0
},{
  name: "Damage +3",
  cost: 100,
  damage: 3,
  armor: 0
},{
  name: "Defense +1",
  cost: 20,
  damage: 0,
  armor: 1
},{
  name: "Defense +2",
  cost: 40,
  damage: 0,
  armor: 2
},{
  name: "Defense +3",
  cost: 80,
  damage: 0,
  armor: 3
}];

var test_boss = {
  name: "boss",
  hp: 12,
  damage: 7,
  armor: 2
};

var test = {
  name: "player",
  hp: 8,
  damage: 5,
  armor: 5
};

var simulate = function(player, enemy) {
  var winner = null;

  var damage_dealt_by_player = Math.max(1, player.damage - enemy.armor);
  var damage_dealt_by_enemy = Math.max(1, enemy.damage - player.armor);

  while (true) {
    //console.log("Player does %d damage", damage_dealt_by_player);
    enemy.hp -= damage_dealt_by_player;
    if (enemy.hp <= 0) return player;

    //console.log("Enemy does %d damage", damage_dealt_by_enemy);
    player.hp -= damage_dealt_by_enemy;
    if (player.hp <= 0) return enemy;

    // console.log("After turn, player is at %d and enemy is at %s", player.hp, enemy.hp);
  };
};

//console.log(simulate(test, boss).name);

var boss = {
  name: "boss",
  hp: 103,
  damage: 9,
  armor: 2
};

var combos = [];

// Build an enumeration of all possible armor, ring, and weapon combos
for (var i=-1; i<armor.length; ++i) {
  for (var j=-1; j<weapons.length; ++j) {
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

        if (j != -1) {
          combo.damage += weapons[j].damage;
          combo.cost += weapons[j].cost;
        }

        if (k != -1) {
          combo.armor += rings1[k].armor;
          combo.damage += rings1[k].damage;
          combo.cost += rings1[k].cost;
        }

        if (l != -1) {
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
  return a.cost - b.cost;
});

console.log("Most expensive: %s", combos[combos.length-1].cost);
console.log("Least expensive: %s", combos[0].cost);

for (var i=0; i<combos.length; ++i) {

  // Reassert boss hp
  boss.hp = 103;

  if (simulate(combos[i], boss).name == undefined) {
    break;
  }
};

console.log("Cheapest winner is %d", combos[i].cost);
