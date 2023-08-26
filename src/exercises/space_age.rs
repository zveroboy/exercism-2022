const EARTH_SECONDS_NUMBER: f64 = 31557600.0;

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! make_planet {
    ($planet: ident, $period: literal) => {
        struct $planet;
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.0 as f64 / (EARTH_SECONDS_NUMBER * $period)
            }
        }
    };
}

make_planet!(Mercury, 0.2408467);
make_planet!(Venus, 0.61519726);
make_planet!(Earth, 1.0);
make_planet!(Mars, 1.8808158);
make_planet!(Jupiter, 11.862615);
make_planet!(Saturn, 29.447498);
make_planet!(Uranus, 84.016846);
make_planet!(Neptune, 164.79132);

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
    }
}
#[test]
fn earth_age() {
    let duration = Duration::from(1_000_000_000);
    assert_in_delta(31.69, Earth::years_during(&duration));
}
#[test]
#[ignore]
fn mercury_age() {
    let duration = Duration::from(2_134_835_688);
    assert_in_delta(280.88, Mercury::years_during(&duration));
}
#[test]
#[ignore]
fn venus_age() {
    let duration = Duration::from(189_839_836);
    assert_in_delta(9.78, Venus::years_during(&duration));
}
#[test]
#[ignore]
fn mars_age() {
    let duration = Duration::from(2_129_871_239);
    assert_in_delta(35.88, Mars::years_during(&duration));
}
#[test]
#[ignore]
fn jupiter_age() {
    let duration = Duration::from(901_876_382);
    assert_in_delta(2.41, Jupiter::years_during(&duration));
}
#[test]
#[ignore]
fn saturn_age() {
    let duration = Duration::from(2_000_000_000);
    assert_in_delta(2.15, Saturn::years_during(&duration));
}
#[test]
#[ignore]
fn uranus_age() {
    let duration = Duration::from(1_210_123_456);
    assert_in_delta(0.46, Uranus::years_during(&duration));
}
#[test]
#[ignore]
fn neptune_age() {
    let duration = Duration::from(1_821_023_456);
    assert_in_delta(0.35, Neptune::years_during(&duration));
}
