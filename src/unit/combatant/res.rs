use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

/// Contants of sum of resource that will be consumed.
/// It's sum, and not only parts, nutritions, and electrics.
pub struct Res(pub Decimal);

// the numbers are sum of links.
pub const EMILY_2: Res = Res(dec!(99));
pub const EMILY_3: Res = Res(dec!(111));
pub const ALEXANDRA: Res = Res(dec!(59));
// end char is lank.
pub const MIGHTY_R_S: Res = Res(dec!(39));
pub const MIGHTY_R_SS: Res = Res(dec!(48));
pub const TOMY_WORKER: Res = Res(dec!(42));
