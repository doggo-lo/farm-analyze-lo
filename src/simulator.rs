mod stage;
mod unit;

use rust_decimal::Decimal;
pub use stage::Stage;
pub use unit::{
  accompanyings, Chicks, HeadCount, UnitIO, UnitIOMaker,
};

/// A dataclass of simulated result.
/// The each value is floored.
pub struct Detail {
  pub total_exp: Decimal,
  pub total_res: Decimal,
  pub exp_per_res: Decimal,
}

// it calcs some element at the same time.
pub fn simulate(unit_io: UnitIO, stage: Stage) -> Detail {
  let laps = stage.get_laps();

  let unit_exp = unit_io.sum_exp(stage.get_exp());
  let unit_res = unit_io.sum_res();
  let exp_per_res = unit_exp / unit_res;
  let exp_per_res = exp_per_res.floor();

  let total_exp = unit_exp * laps;
  let total_res = unit_res * laps;

  Detail {
    total_exp,
    total_res,
    exp_per_res,
  }
}

#[cfg(test)]
mod simulate {
  use super::*;
  use chrono::Duration;
  use rstest::*;
  use rust_decimal_macros::dec;

  #[fixture]
  fn detail() -> Detail {
    let count = HeadCount::new(5);
    let unit_io = UnitIOMaker::new(count)
      .add(accompanyings::EMILY_2)
      .fill(Chicks::Cover(dec!(50)));
    let laptime = Duration::minutes(1);
    let total_time = Duration::hours(1);
    let stage = Stage::new(laptime, total_time, dec!(10000));
    simulate(unit_io, stage)
  }

  #[rstest]
  fn total_exp(detail: Detail) {
    let total = detail.total_exp;
    // 52800 from test of unit
    let expected = dec!(52800) * dec!(60);
    assert_eq!(total, expected);
  }

  #[rstest]
  fn total_res(detail: Detail) {
    let total = detail.total_res;
    // expected without 60 from test of unit
    let res_emily = accompanyings::EMILY_2.res.0;
    let expected = (res_emily + Decimal::new(50 * 4, 0)) * dec!(60);
    assert_eq!(total, expected);
  }

  fn exp_per_res(detail: Detail) {
    let balance = detail.exp_per_res;
    // 52800/299
    let expected = dec!(176);
    assert_eq!(balance, expected);
  }
}
