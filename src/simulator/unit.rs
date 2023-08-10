mod combatant;
mod count;
use std::ops::Range;

pub use combatant::accompanyings;
use combatant::{buffs, Accompanying, Chick};
pub use count::HeadCount;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

mod chicks {
  //! To fill remain of unit with chicks.
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
  /// glue code.
  pub fn fill(&self, range: Range<usize>) -> Vec<Chick> {
    match self {
      Self::Cover(_) => chicks::covers(range),
      Self::Uncover(_) => chicks::uncovers(range),
    }
  }
  /// glue code.
  pub fn res_av(&self) -> Decimal {
    match self {
      Self::Cover(res) => *res,
      Self::Uncover(res) => *res,
    }
  }
}

/// The builder for out of res and in of exp.
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
  /// You should pass constant of accompanying
  /// from accompanyings modules. e.g. EMILY_2.
  pub fn add(&mut self, accompanying: Accompanying) -> &mut Self {
    let Accompanying { res, skill_buff } = accompanying;
    self.res += res.0;
    self.skill_buff += skill_buff.0;
    self.count.inc();
    self
  }
  /// fill with chicks to headcount.
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

/// Details for unit io. this does'nt have some special logics.
/// It's data class.
pub struct UnitIO {
  chicks: Vec<Chick>,
  skill_buff: Decimal,
  res: Decimal,
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
mod unitio_variant {
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
