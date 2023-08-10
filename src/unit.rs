mod combatant;
mod count;
use std::{fmt::DebugList, ops::Range};

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

/// it is cover if there is chick who has
/// leader-buff and improved-learning-system.
/// So others are uncover.
/// Add, it has average of total-resource.
pub enum Chicks {
  Cover(Decimal),
  Uncover(Decimal),
}

impl Chicks {
  pub fn fill(&self, range: Range<usize>) -> Vec<Chick> {
    match self {
      Self::Cover(_) => chicks::covers(range),
      Self::Uncover(_) => chicks::uncovers(range),
    }
  }
  pub fn res_av(&self) -> Decimal {
    match self {
      Self::Cover(res) => *res,
      Self::Uncover(res) => *res,
    }
  }
}

pub struct UnitIOMaker {
  res: Decimal,
  skill_buff: Decimal,
  count: HeadCount,
}

impl UnitIOMaker {
  pub fn new(count: HeadCount) -> Self {
    UnitIOMaker {
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
  pub fn fill(&self, filler: Chicks) -> UnitIO {
    let chicks = filler.fill(self.count.remain_range());
    let num_chicks = Decimal::new(chicks.len() as i64, 0);
    let res = num_chicks * filler.res_av() + self.res;
    UnitIO {
      chicks,
      res,
      skill_buff: self.skill_buff,
    }
  }
}

pub struct UnitIO {
  chicks: Vec<Chick>,
  res: Decimal,
  skill_buff: Decimal,
}

impl UnitIO {
  pub fn sum_exp(&self, base: Decimal) -> Decimal {
    let exp: Decimal = self
      .chicks
      .iter()
      .map(|x| x.calc_exp(base, self.skill_buff))
      .sum();
    exp
  }
  pub fn sum_res(&self) -> Decimal {
    self.res
  }
}

#[cfg(test)]
mod unit_variant {
  use super::*;
  use rstest::*;

  #[fixture]
  fn unit() -> UnitIO {
    let count = HeadCount::new(5);
    UnitIOMaker::new(count)
      .add(accompanyings::EMILY_2)
      .fill(Chicks::Cover(dec!(50)))
  }

  #[rstest]
  fn sum_exp(unit: UnitIO) {
    let exp = unit.sum_exp(dec!(10000));
    // The covered is 15300, the normal is 12500. and there are three normals.
    // Equals, 15300+12500*3=52800
    let expected = dec!(52800);
    assert_eq!(exp, expected);
  }

  #[rstest]
  fn sum_res(unit: UnitIO) {
    let res = unit.sum_res();
    let res_emily = accompanyings::EMILY_2.res.0;
    let expected = res_emily + Decimal::new(50 * 4, 0);
    assert_eq!(res, expected);
  }
}
