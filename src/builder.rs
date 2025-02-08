use crate::SeedPhrase::SeedPhrase;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

pub trait PreSuiteTrait {}

pub trait PreTestTrait {}

pub trait PostTestTrait {}

pub trait PostSuiteTrait {}

pub struct NoOpSetup;

impl PreSuiteTrait for NoOpSetup {}
impl PreTestTrait for NoOpSetup {}
impl PostTestTrait for NoOpSetup {}
impl PostSuiteTrait for NoOpSetup {}

pub struct RapidFuzzBuilder<PreSuite, PreTest, PostTest, PostSuite> {
    pre_suite: PreSuite,
    pre_test: PreTest,
    post_test: PostTest,
    post_suite: PostSuite,
}

impl RapidFuzzBuilder<(), (), (), ()> {
    pub fn new() -> RapidFuzzBuilder<(), (), (), ()> {
        RapidFuzzBuilder {
            pre_suite: (),
            pre_test: (),
            post_test: (),
            post_suite: (),
        }
    }
}

impl<PreTest, PostTest, PostSuite> RapidFuzzBuilder<(), PreTest, PostTest, PostSuite> {
    pub fn with_pre_suite<PreSuite: PreSuiteTrait>(
        self,
        pre_suite: PreSuite,
    ) -> RapidFuzzBuilder<PreSuite, PreTest, PostTest, PostSuite> {
        RapidFuzzBuilder {
            pre_suite,
            pre_test: self.pre_test,
            post_test: self.post_test,
            post_suite: self.post_suite,
        }
    }

    pub fn without_pre_suite(self) -> RapidFuzzBuilder<NoOpSetup, PreTest, PostTest, PostSuite> {
        RapidFuzzBuilder {
            pre_suite: NoOpSetup,
            pre_test: self.pre_test,
            post_test: self.post_test,
            post_suite: self.post_suite,
        }
    }
}

impl<PreSuite, PostTest, PostSuite> RapidFuzzBuilder<PreSuite, (), PostTest, PostSuite> {
    pub fn with_pre_test<PreTest: PreTestTrait>(
        self,
        pre_test: PreTest,
    ) -> RapidFuzzBuilder<PreSuite, PreTest, PostTest, PostSuite> {
        RapidFuzzBuilder {
            pre_suite: self.pre_suite,
            pre_test,
            post_test: self.post_test,
            post_suite: self.post_suite,
        }
    }

    pub fn without_pre_test(self) -> RapidFuzzBuilder<PreSuite, NoOpSetup, PostTest, PostSuite> {
        RapidFuzzBuilder {
            pre_suite: self.pre_suite,
            pre_test: NoOpSetup,
            post_test: self.post_test,
            post_suite: self.post_suite,
        }
    }
}

impl<PreSuite, PreTest, PostSuite> RapidFuzzBuilder<PreSuite, PreTest, (), PostSuite> {
    pub fn with_post_test<PostTest: PostTestTrait>(
        self,
        post_test: PostTest,
    ) -> RapidFuzzBuilder<PreSuite, PreTest, PostTest, PostSuite> {
        RapidFuzzBuilder {
            pre_suite: self.pre_suite,
            pre_test: self.pre_test,
            post_test,
            post_suite: self.post_suite,
        }
    }

    pub fn without_post_test(self) -> RapidFuzzBuilder<PreSuite, PreTest, NoOpSetup, PostSuite> {
        RapidFuzzBuilder {
            pre_suite: self.pre_suite,
            pre_test: self.pre_test,
            post_test: NoOpSetup,
            post_suite: self.post_suite,
        }
    }
}

impl<PreSuite, PreTest, PostTest> RapidFuzzBuilder<PreSuite, PreTest, PostTest, ()> {
    pub fn with_post_suite<PostSuite: PostSuiteTrait>(
        self,
        post_suite: PostSuite,
    ) -> RapidFuzzBuilder<PreSuite, PreTest, PostTest, PostSuite> {
        RapidFuzzBuilder {
            pre_suite: self.pre_suite,
            pre_test: self.pre_test,
            post_test: self.post_test,
            post_suite,
        }
    }

    pub fn without_post_suite(self) -> RapidFuzzBuilder<PreSuite, PreTest, PostTest, NoOpSetup> {
        RapidFuzzBuilder {
            pre_suite: self.pre_suite,
            pre_test: self.pre_test,
            post_test: self.post_test,
            post_suite: NoOpSetup,
        }
    }
}

impl<PreSuite, PreTest, PostTest, PostSuite>
    RapidFuzzBuilder<PreSuite, PreTest, PostTest, PostSuite>
where
    PreSuite: PreSuiteTrait,
    PreTest: PreTestTrait,
    PostTest: PostTestTrait,
    PostSuite: PostSuiteTrait,
{
    pub fn run(&self) {
        let args = parse_clap();
        let cases = match args.seed {
            Some(seed) => (0..=0)
                .map(|_| (seed, ChaChaRng::from_seed(seed.into_bytes())))
                .collect::<Vec<_>>(),
            None => {
                let run_range = match args.max_iterations {
                    None => 0u128..u128::MAX,
                    Some(max) => 0..max,
                };
                run_range
                    .map(|_| {
                        let seed = rand::random::<[u8; 32]>();
                        (
                            SeedPhrase::from_bytes(&seed).unwrap(),
                            ChaChaRng::from_seed(seed),
                        )
                    })
                    .collect::<Vec<_>>()
            }
        };
        for (iteration, (seed, rng)) in cases.into_iter().enumerate() {
            self.run_test(seed, rng);
        }
    }

    fn run_test(&self, seed: SeedPhrase, mut rng: ChaChaRng) {
        if rng.next_u32() % 5 == 0 {
            panic!("Test failed for {}", seed.to_string());
        }
        println!("Test passed for {}", seed.to_string());
    }
}

pub fn parse_clap() -> DeterministicSimulationArgs {
    // TODO
    DeterministicSimulationArgs {
        // If you want a failing test (the first u32 is %5 == 0), use this seed:
        // seed: Some(
        //     SeedPhrase::parse("1b25ed3ce3f95840b72680799128e3c3f00d9d987787909a8bf879cccf271d92")
        //         .unwrap(),
        // ),
        seed: None,
        max_iterations: Some(7),
    }
}

pub struct DeterministicSimulationArgs {
    seed: Option<SeedPhrase>,
    max_iterations: Option<u128>,
}
