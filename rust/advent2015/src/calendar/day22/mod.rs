
trait Spell {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant);
  fn cost(&self) -> u8;
  fn name(&self) -> &'static str;
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

static MAGIC_MISSILE:MagicMissile = MagicMissile { cost: 53 };
static DRAIN:Drain = Drain { cost: 73 };
static SHIELD:Shield = Shield { cost: 113 };
static POISON:Poison = Poison { cost: 173 };
static RECHARGE:Recharge = Recharge { cost: 229 };

#[allow(unused_variables)]
impl Spell for MagicMissile {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    target.apply_damage(4);
  }

  fn cost(&self) -> u8 {
    self.cost
  }

  fn name(&self) -> &'static str {
    "MagicMissile"
  }
}

#[allow(unused_variables)]
impl Spell for Drain {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    caster.current_hp += 2;
    target.apply_damage(2);
  }

  fn cost(&self) -> u8 {
    self.cost
  }

  fn name(&self) -> &'static str {
    "Drain"
  }
}

#[allow(unused_variables)]
impl Spell for Shield {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    caster.shield_remaining = 6;
  }

  fn cost(&self) -> u8 {
    self.cost
  }

  fn name(&self) -> &'static str {
    "Shield"
  }
}

#[allow(unused_variables)]
impl Spell for Poison {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    target.poison_remaining = 6;
  }

  fn cost(&self) -> u8 {
    self.cost
  }

  fn name(&self) -> &'static str {
    "Poison"
  }
}

#[allow(unused_variables)]
impl Spell for Recharge {
  fn cast(&self, caster:&mut Combatant, target:&mut Combatant) {
    caster.recharge_remaining = 5;
  }

  fn cost(&self) -> u8 {
    self.cost
  }

  fn name(&self) -> &'static str {
    "Recharge"
  }
}

#[derive(Debug)]
struct Combatant {
  hp: u8,
  current_hp: i8,
  damage: u8,
  armor: u8,
  mana: u16,
  current_mana: u16,
  shield_remaining: u8,
  poison_remaining: u8,
  recharge_remaining: u8,
  hardmode: bool,
}

impl Combatant {
  fn new_boss(hp: u8, damage: u8) -> Self {
    Combatant {
      hp: hp,
      current_hp: hp as i8,
      armor: 0,
      damage: damage,
      mana: 0,
      current_mana: 0,
      shield_remaining: 0,
      poison_remaining: 0,
      recharge_remaining: 0,
      hardmode: false,
    }
  }

  fn new_player(hp: u8, mana: u16) -> Self {
    Combatant {
      hp: hp,
      current_hp: hp as i8,
      armor: 0,
      damage: 0,
      mana: mana,
      current_mana: mana,
      shield_remaining: 0,
      poison_remaining: 0,
      recharge_remaining: 0,
      hardmode: false,
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
    }

    if self.shield_remaining == 0 {
      self.armor = 0;
    }

    if self.recharge_remaining > 0 {
      self.recharge_remaining -= 1;
      self.current_mana += 101;
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
    self.current_mana = self.mana;
    self.shield_remaining = 0;
    self.poison_remaining = 0;
  }
}

fn fight (player: &mut Combatant, boss: &mut Combatant, spells: &Vec<Box<&Spell>>) -> bool {
  
  for spell in spells {

    if player.hardmode {
      player.current_hp -= 1;
      // Check player deadness
      if player.current_hp <= 0 {
        return false;
      }
    }

    // Apply player effects
    player.apply_player_effects();

    // Apply boss effects
    boss.apply_boss_effects();

    if player.current_mana < spell.cost() as u16 {
      return false;
    }

    // Cast
    spell.cast(player, boss);

    player.current_mana -= spell.cost() as u16;

    //println!("After player turn\nplayer: {:?}\nboss: {:?}", player, boss);

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

    //println!("\tAfter boss turn\n\tplayer: {:?}\n\tboss: {:?}", player, boss);

    // Check player deadness
    if player.current_hp <= 0 {
      return false;
    }
  }

  false
}

fn permute_n_spells(spells: &mut Vec<Box<&Spell>>,
  player:&mut Combatant, boss:&mut Combatant,
  n: u8, max: u8, lowest: &mut i16) {

  for spell_number in 0..5 {
    let spell:Box<&Spell> = match spell_number {
      0 => Box::new(&MAGIC_MISSILE),
      1 => Box::new(&DRAIN),
      2 => Box::new(&SHIELD),
      3 => Box::new(&POISON),
      4 => Box::new(&RECHARGE),
      _ => panic!("Math failed me"),
    };

    spells[(n as usize)-1] = spell;

    if n == max {
      // We have a full populated spell array, so run this mug.
      if fight(player, boss, spells) {
        let mut cost:u16 = 0;
        for my_spell in spells.iter() {
          cost += my_spell.cost() as u16;
        }

        if *lowest == -1 || cost < *lowest as u16 {
          *lowest = cost as i16;
        }
      }

      // After every fight, reset player and boss
      boss.reset();
      player.reset();
    } else {
      permute_n_spells(spells, player, boss, n+1, max, lowest);
    }
  }
}

fn permute(player: &mut Combatant, boss: &mut Combatant) -> u16 {
  let mut spell_count:u8 = 1;
  loop {
    let mut spells:Vec<Box<&Spell>> = vec!();
    spells.resize(spell_count as usize, Box::new(&MAGIC_MISSILE));
    let mut lowest:i16 = -1;
    permute_n_spells(&mut spells, player, boss, 1, spell_count, &mut lowest);
    if lowest != -1 {
      return lowest as u16;
    }

    println!("No matches at size {}", spell_count);

    spell_count += 1;
  }
}

fn part1 (_: String) -> String {
  let mut boss:Combatant = Combatant::new_boss(71, 10);

  let mut player:Combatant = Combatant::new_player(50, 500);

  let lowest_cost:u16 = permute(&mut player, &mut boss);
  return lowest_cost.to_string();
}

fn part2 (_: String) -> String {
  let mut boss:Combatant = Combatant::new_boss(71, 10);

  let mut player:Combatant = Combatant::new_player(50, 500);
  player.hardmode = true;

  let lowest_cost:u16 = permute(&mut player, &mut boss);
  return lowest_cost.to_string();
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

//
// These test cases just take way too long to run.
//

fn test_part1() {
  let day = fill();
  assert_eq!((day.part1.run)(day.input.to_string()), "1824".to_string());
}

fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "1937".to_string());
}
