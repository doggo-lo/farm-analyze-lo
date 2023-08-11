# farm-analyze-lo

The calc library for efficient about exp and resource at the game called LastOrigin.

# Important

The repository has the possibility that this will be deleted.
Because this is fan made and I'll delete this if probrems occurred with the official group of LastOrigin.
Add, this library is only for jp. This may not use with KR ver.

# Examples

```rust

// First, decide number of unit menber.
let count = HeadCount::new(5);
// Make input(exp)/output(resource) detail of unit.
let unit_io = UnitIOMaker::new(count)
  .add(accompanyings::EMILY_2)
  .fill(Chicks::Cover(dec!(50)));
// Decide a times.
let laptime = Duration::minutes(1);
let total_time = Duration::hours(1);
// It needs stage's detail, too.
let stage = Stage::new(laptime, total_time, dec!(10000));
// That's the end.
let detail = simulate(unit_io, stage);

```

# What this do

The library works well to analyze about exp per resouces at the farming.
For example, about OS called Learning system on Emily's 5-8Ex or Mighty-R's 7-7.
However, the clac result has an error by some factors like rounding off.
You can use the result as rough guess.

# Dependencies

In API, this use `chrono` crate to handle time in sligntly larger units than `std::time::Duration`.
You can pass times to the library at hours or minutes by it.
Add, this use `rust_decimal` crate to handle float value with smaller errors than default f64.

# References

* The formula about exp with some buffs from combatant's skill and gear them.
  * [전투 결과 분석실의 경험치 계산](https://arca.live/b/breaking/6491839)
* The resources and values of buffs.
  * [Last Origin (ラストオリジン) 日本版攻略wiki](https://arca.live/b/breaking/6491839)

# License

MIT
