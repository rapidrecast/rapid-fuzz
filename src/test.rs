use rand::RngCore;
use rapid_fuzz::arbitrary_util::GenerateArbitrary;
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
    fn run<RNG: RngCore>(&self, _suite: &Suite, _test: &Test, mut rng: RNG) {
        let data: (u64, u64) = (
            u64::generate_owned_arbitrary_data(&mut rng)
                .arbitrary()
                .unwrap(),
            u64::generate_owned_arbitrary_data(&mut rng)
                .arbitrary()
                .unwrap(),
        );
        let data2: (i32, i32) = (
            i32::generate_owned_arbitrary_data(&mut rng)
                .arbitrary()
                .unwrap(),
            i32::generate_owned_arbitrary_data(&mut rng)
                .arbitrary()
                .unwrap(),
        );
        // Define unlucky numbers for each type
        let unlucky_u64: [u64; 2] = [1331, 1771];
        let unlucky_i32: [i32; 2] = [1331, 1771];

        // Check if any values are in the unlucky lists
        assert!(
            !unlucky_u64.iter().any(|&n| data.0 % n == 0),
            "Data.0 ({}) was unlucky",
            data.0,
        );
        assert!(
            !unlucky_u64.iter().any(|&n| data.1 % n == 0),
            "Data.1 ({}) was unlucky",
            data.1
        );
        assert!(
            !unlucky_i32.iter().any(|&n| data2.0 % n == 0),
            "Data2.0 ({}) was unlucky",
            data2.0
        );
        assert!(
            !unlucky_i32.iter().any(|&n| data2.1 % n == 0),
            "Data2.1 ({}) was unlucky",
            data2.1
        );
    }
}
