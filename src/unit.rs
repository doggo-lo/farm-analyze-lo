mod combatant;
mod count;
use std::ops::Range;

pub use combatant::accompanyings;
use combatant::{buffs, Accompanying, Chick};
pub use count::HeadCount;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

mod chicks {
  use super::*;
  use std::ops::Range;
  fn normal() -> Chick {
    Chick::new(buffs::leader::FORROWER, buffs::os::NORMAL)
  }
  pub fn covers(range: Range<usize>) -> Vec<Chick> {
    vec![
      Chick::new(buffs::leader::LEADER, buffs::os::IMPROVED),
      normal(),
      normal(),
      normal(),
    ][range]
      .to_vec()
  }
  pub fn uncovers(range: Range<usize>) -> Vec<Chick> {
    vec![
      Chick::new(buffs::leader::LEADER, buffs::os::NORMAL),
      Chick::new(buffs::leader::FORROWER, buffs::os::IMPROVED),
      normal(),
      normal(),
    ][range]
      .to_vec()
  }
}

pub enum Chicks {
  Cover,
  Uncover,
}

impl Chicks {
  pub fn fill(self, range: Range<usize>) -> Vec<Chick> {
    match self {
      Self::Cover => chicks::covers(range),
      Self::Uncover => chicks::uncovers(range),
    }
  }
}

pub struct UnitMaker {
  res: Decimal,
  skill_buff: Decimal,
  count: HeadCount,
}

impl UnitMaker {
  pub fn new(count: HeadCount) -> Self {
    UnitMaker {
      res: dec!(0),
      skill_buff: dec!(0),
      count,
    }
  }
  pub fn add(&mut self, accompanying: Accompanying) -> &mut Self {
    let Accompanying { res, skill_buff } = accompanying;
    self.res += res.0;
    self.skill_buff += skill_buff.0;
    self.count.inc();
    self
  }
  pub fn fill(&self, filler: Chicks) -> Unit {
    let chicks = filler.fill(self.count.remain_range());
    Unit {
      chicks,
      res: self.res,
      skill_buff: self.skill_buff,
    }
  }
}

pub struct Unit {
  chicks: Vec<Chick>,
  res: Decimal,
  skill_buff: Decimal,
}

impl Unit {
  pub fn sum_exp(&self, base: Decimal) -> Decimal {
    let exp: Decimal = self
      .chicks
      .iter()
      .map(|x| x.calc_exp(base, self.skill_buff))
      .sum();
    exp
  }
}
