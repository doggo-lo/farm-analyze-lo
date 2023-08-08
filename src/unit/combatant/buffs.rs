use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

pub mod leader {
  //! Constants for parameters about calcing exp.
  //! Add, this is the buff with specifying leader.
  use super::*;
  #[derive(Clone)]
  pub struct LeaderBuff(pub Decimal);
  pub const LEADER: LeaderBuff = LeaderBuff(dec!(1.2));
  pub const FORROWER: LeaderBuff = LeaderBuff(dec!(1));
}

pub mod os {
  //! Constants for parameters about calcing exp.
  //! Add, this is the buff with the gear "Learning system".
  use super::*;
  #[derive(Clone)]
  pub struct OsBuff(pub Decimal);
  pub const IMPROVED: OsBuff = OsBuff(dec!(1.275));
  pub const NORMAL: OsBuff = OsBuff(dec!(1.25));
}

pub mod skill {
  //! Constants for parameters about calcing exp.
  //! Add, this is the buff by the other's skill. e.g. Alexandra, Mighty-R.
  //! And, it is only that the confficent is without 1, because,
  //! it will be added up with other buff, and it can overlap on other skill-buff.
  use super::*;
  pub struct SkillBuff(pub Decimal);
  pub const ALEXANDRA: SkillBuff = SkillBuff(dec!(0.4025));
  pub const MIGHTLY_R: SkillBuff = SkillBuff(dec!(0.38));
  pub const TOMY_WORKER_BIO: SkillBuff = SkillBuff(dec!(0.3));
  pub const TOMY_WORKER_AGS: SkillBuff = SkillBuff(dec!(0.6));
  pub const NONE: SkillBuff = SkillBuff(dec!(0.0));
}
