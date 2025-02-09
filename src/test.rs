use rand::RngCore;
use rand_chacha::ChaCha20Rng;
use rapid_fuzz::arbitrary_util::GenerateArbitrary;
use rapid_fuzz::{PreTestTrait, ProgramTrait, RapidFuzzBuilder};

pub fn main() {
    let builder = RapidFuzzBuilder::new()
        .without_pre_suite()
        .with_pre_test(RandomNumbersPreTest::new())
        .without_post_test()
        .without_post_suite()
        .with_test_program(RandomNumbersAndCompareProgram {});
    builder.run();
}

pub struct RandomNumbersPreTest<Suite> {
    _phantom: std::marker::PhantomData<Suite>,
}

impl<Suite> RandomNumbersPreTest<Suite> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Suite> PreTestTrait<Suite, RandomNumbersData> for RandomNumbersPreTest<Suite> {
    fn setup_test(&self, _suite: &Suite, rng: &mut ChaCha20Rng) -> RandomNumbersData {
        let data: (u64, u64) = (
            u64::generate_owned_arbitrary_data(rng).arbitrary().unwrap(),
            u64::generate_owned_arbitrary_data(rng).arbitrary().unwrap(),
        );
        let data2: (i32, i32) = (
            i32::generate_owned_arbitrary_data(rng).arbitrary().unwrap(),
            i32::generate_owned_arbitrary_data(rng).arbitrary().unwrap(),
        );
        RandomNumbersData { data, data2 }
    }
}

pub struct RandomNumbersData {
    pub data: (u64, u64),
    pub data2: (i32, i32),
}

pub struct RandomNumbersAndCompareProgram {}

impl<Suite> ProgramTrait<Suite, RandomNumbersData> for RandomNumbersAndCompareProgram {
    fn run<RNG: RngCore>(&self, _suite: &Suite, test: &RandomNumbersData, _rng: RNG) {
        // Define unlucky numbers for each type
        fn is_unlucky_u64(n: u64) -> bool {
            let unlucky_u64: [u64; 2] = [1331, 1771];
            unlucky_u64.iter().any(|&unlucky| n % unlucky == 0)
        }

        fn is_unlucky_i32(n: i32) -> bool {
            let unlucky_i32: [i32; 2] = [1331, 1771];
            unlucky_i32.iter().any(|&unlucky| n % unlucky == 0)
        }

        // Check if any values are in the unlucky lists
        assert!(
            !is_unlucky_u64(test.data.0),
            "Data.0 ({}) was unlucky",
            test.data.0,
        );
        assert!(
            !is_unlucky_u64(test.data.1),
            "Data.1 ({}) was unlucky",
            test.data.1
        );
        assert!(
            !is_unlucky_i32(test.data2.0),
            "Data2.0 ({}) was unlucky",
            test.data2.0
        );
        if is_unlucky_i32(test.data2.0) || is_unlucky_i32(test.data2.1) {
            // Cause a massive allocation
            let mut data = Vec::with_capacity(usize::MAX);
            data.push(3);
        }
    }
}
