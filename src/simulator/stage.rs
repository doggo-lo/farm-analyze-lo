use chrono::Duration;
use rust_decimal::Decimal;

/// dataclass for stage to farm.
pub struct Stage {
  laps: Decimal,
  exp_base: Decimal,
}

fn to_decimal_sec(duration: Duration) -> Decimal {
  let sec = duration.num_seconds();
  Decimal::new(sec, 0)
}

impl Stage {
  // About laps number, it will be floored.
  pub fn new(
    laptime: Duration,
    total_time: Duration,
    exp_base: Decimal,
  ) -> Stage {
    let laps = to_decimal_sec(total_time) / to_decimal_sec(laptime);
    let laps = laps.floor();
    Stage { laps, exp_base }
  }
  pub fn get_laps(&self) -> Decimal {
    self.laps
  }
  pub fn get_exp(&self) -> Decimal {
    self.exp_base
  }
}

#[cfg(test)]
mod stage_variant {
  use super::*;
  use rust_decimal_macros::dec;

  #[test]
  fn laps() {
    let laptime = Duration::minutes(1);
    let total_time = Duration::hours(1);
    let stage = Stage::new(laptime, total_time, dec!(100));
    let laps = stage.get_laps();
    let expected = dec!(60);
    assert_eq!(laps, expected);
  }
}
