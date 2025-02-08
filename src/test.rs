use rand::RngCore;
use rapid_fuzz::{ProgramTrait, RapidFuzzBuilder};

pub fn main() {
    let builder = RapidFuzzBuilder::new()
        .without_pre_suite()
        .without_pre_test()
        .without_post_test()
        .without_post_suite()
        .with_test_program(RandomNumbersAndCompareProgram {});
    builder.run();
}

pub struct RandomNumbersAndCompareProgram {}

impl<Suite, Test> ProgramTrait<Suite, Test> for RandomNumbersAndCompareProgram {
    fn run<RNG: RngCore>(&self, suite: &Suite, test: &Test, mut rng: RNG) {
        let data: (u64, u64) = (
            u64::generate_arbitrary(&mut rng).arbitrary(),
            u64::generate_arbitrary(&mut rng).arbitrary(),
        );
        let data2: (i32, i32) = (
            i32::generate_arbitrary(&mut rng).arbitrary(),
            i32::generate_arbitrary(&mut rng).arbitrary(),
        );
        assert!(data.0 as i64 > data2.0 as i64);
        assert!(data.1 as i64 > data2.1 as i64);
    }
}
