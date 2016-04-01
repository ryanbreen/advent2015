
trait Spell {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant);
  fn cost(&self) -> u8;
}

#[derive(Debug)]
struct MagicMissile {
  cost: u8,
}

#[derive(Debug)]
struct Drain {
  cost: u8,
}

#[derive(Debug)]
struct Shield {
  cost: u8,
}

#[derive(Debug)]
struct Poison {
  cost: u8,
}

#[derive(Debug)]
struct Recharge {
  cost: u8,
}

const MAGIC_MISSILE:MagicMissile = MagicMissile { cost: 53 };
const DRAIN:Drain = Drain { cost: 73 };
const SHIELD:Shield = Shield { cost: 113 };
const POISON:Poison = Poison { cost: 173 };
const RECHARGE:Recharge = Recharge { cost: 229 };

impl Spell for MagicMissile {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    target.apply_damage(4);
  }

  fn cost(&self) -> u8 {
    self.cost
  }
}

impl Spell for Drain {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    caster.current_hp += 2;
    target.apply_damage(2);
  }

  fn cost(&self) -> u8 {
    self.cost
  }
}

impl Spell for Shield {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    caster.shield_remaining = 6;
  }

  fn cost(&self) -> u8 {
    self.cost
  }
}

impl Spell for Poison {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    target.poison_remaining = 6;
  }

  fn cost(&self) -> u8 {
    self.cost
  }
}

impl Spell for Recharge {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    caster.recharge_remaining = 5;
  }

  fn cost(&self) -> u8 {
    self.cost
  }
}

#[derive(Debug)]
struct Combatant {
  hp: u8,
  current_hp: i8,
  damage: u8,
  armor: u8,
  mana: u16,
  shield_remaining: u8,
  poison_remaining: u8,
  recharge_remaining: u8,
}

impl Combatant {
  fn new_boss(hp: u8, damage: u8) -> Self {
    Combatant {
      hp: hp,
      current_hp: hp as i8,
      armor: 0,
      damage: damage,
      mana: 0,
      shield_remaining: 0,
      poison_remaining: 0,
      recharge_remaining: 0,
    }
  }

  fn new_player(hp: u8, mana: u16) -> Self {
    Combatant {
      hp: hp,
      current_hp: hp as i8,
      armor: 0,
      damage: 0,
      mana: mana,
      shield_remaining: 0,
      poison_remaining: 0,
      recharge_remaining: 0,
    }
  }

  fn apply_damage(&mut self, damage: u8) {
    if damage < self.armor {
      return;
    }

    self.current_hp -= (damage - self.armor) as i8;
  }

  fn apply_player_effects(&mut self) {

    if self.shield_remaining > 0 {
      self.shield_remaining -= 1;
      self.armor = 7;
    } else if self.shield_remaining == 0 {
      self.armor = 0;
    }

    if self.recharge_remaining > 0 {
      self.recharge_remaining -= 1;
      self.mana += 101;
    }

  }

  fn apply_boss_effects(&mut self) {
    if self.poison_remaining > 0 {
      self.poison_remaining -= 1;
      self.current_hp -= 3;
    }
  }

  fn reset(&mut self) {
    self.current_hp = self.hp as i8;
    self.shield_remaining = 0;
    self.poison_remaining = 0;
  }
}

fn fight (player: &mut Combatant, boss: &mut Combatant, spells: &Vec<Box<Spell>>) -> bool {
  
  for spell in spells {

    // Apply player effects
    player.apply_player_effects();

    // Apply boss effects
    boss.apply_boss_effects();

    if player.mana < spell.cost() as u16 {
      return false;
    }

    // Cast
    spell.cast(player, boss);

    player.mana -= spell.cost() as u16;

    println!("After player turn\nplayer: {:?}\nboss: {:?}", player, boss);

    // Check boss deadness
    if boss.current_hp <= 0 {
      return true;
    }

    // Apply player effects
    player.apply_player_effects();

    // Apply boss effects
    boss.apply_boss_effects();

    // Check boss deadness
    if boss.current_hp <= 0 {
      return true;
    }

    // Boss attack
    player.apply_damage(boss.damage);

    println!("\tAfter boss turn\n\tplayer: {:?}\n\tboss: {:?}", player, boss);

    // Check player deadness
    if player.current_hp <= 0 {
      return false;
    }
  }

  false
}

fn part1 (_: String) -> String {
  let mut boss:Combatant = Combatant::new_boss(14, 8);

  let mut player:Combatant = Combatant::new_player(10, 250);

  let mut spells:Vec<Box<Spell>> = vec!();
/*
  spells.push(Box::new(POISON));
  spells.push(Box::new(MAGIC_MISSILE));
*/
  spells.push(Box::new(RECHARGE));
  spells.push(Box::new(SHIELD));
  spells.push(Box::new(DRAIN));
  spells.push(Box::new(POISON));
  spells.push(Box::new(MAGIC_MISSILE));

  println!("{}", fight(&mut player, &mut boss, &mut spells));

  let mut lowest_cost:u16 = 65535;
  return lowest_cost.to_string();
}

fn part2 (_: String) -> String {
  let mut highest_cost:u16 = 0;

  return highest_cost.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "".to_string(),
    part1: super::Puzzle {
      run: part1,
    },
    part2: super::Puzzle {
      run: part2,
    }
  };
}

#[test]
fn test_part1() {
  let day = fill();
  assert_eq!((day.part1.run)(day.input.to_string()), "121".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "201".to_string());
}
