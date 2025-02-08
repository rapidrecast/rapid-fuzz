use crate::seed_phrase::SeedPhrase;
use clap::{Arg, Command};
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
        for (_iteration, (seed, rng)) in cases.into_iter().enumerate() {
            self.run_test(seed, rng);
        }
    }

    fn run_test(&self, seed: SeedPhrase, mut rng: ChaChaRng) {
        if rng.next_u32() % 7 == 0 {
            panic!("Test failed for {}", seed.to_string());
        }
        println!("Test passed for {}", seed.to_string());
    }
}

pub fn parse_clap() -> DeterministicSimulationArgs {
    let matches = Command::new("Deterministic Simulation")
        .about("Runs a deterministic simulation with optional parameters")
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .value_name("HEX")
                .help("Hex-encoded 64-character seed phrase"),
        )
        .arg(
            Arg::new("max_iterations")
                .short('m')
                .long("max-iterations")
                .value_name("N")
                .help("Maximum number of iterations")
                .default_value("7"),
        )
        .get_matches();

    let seed = matches
        .get_one::<String>("seed")
        .map(|s| SeedPhrase::parse(s).unwrap());
    let max_iterations = matches
        .get_one::<String>(&"max_iterations")
        .and_then(|n| n.parse::<u128>().ok());

    DeterministicSimulationArgs {
        seed,
        max_iterations,
    }
}

pub struct DeterministicSimulationArgs {
    seed: Option<SeedPhrase>,
    max_iterations: Option<u128>,
}
