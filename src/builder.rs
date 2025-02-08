use crate::seed_phrase::SeedPhrase;
use clap::{Arg, Command};
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use std::marker::PhantomData;

pub trait PreSuiteTrait<Suite> {
    fn setup_suite(&self) -> Suite;
}

pub trait PreTestTrait<Suite, Test> {
    fn setup_test(&self, suite: &Suite) -> Test;
}

pub trait PostTestTrait<Suite, Test> {
    fn teardown_test(&self, suite: &Suite, test: Test);
}

pub trait PostSuiteTrait<Suite> {
    fn teardown_suite(&self, suite: Suite);
}

pub trait ProgramTrait<Suite, Test> {
    fn run<RNG: RngCore>(&self, suite: &Suite, test: &Test, rng: RNG);
}

pub struct NoOpSetup;

impl PreSuiteTrait<()> for NoOpSetup {
    fn setup_suite(&self) -> () {}
}
impl<Suite> PreTestTrait<Suite, ()> for NoOpSetup {
    fn setup_test(&self, _suite: &Suite) -> () {}
}

impl<Suite> PostTestTrait<Suite, ()> for NoOpSetup {
    fn teardown_test(&self, _suite: &Suite, _test: ()) {}
}
impl PostSuiteTrait<()> for NoOpSetup {
    fn teardown_suite(&self, _suite: ()) {}
}

pub struct RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, TestProgram, PostTest, PostSuite> {
    suite: PhantomData<Suite>,
    test: PhantomData<Test>,
    pre_suite: PreSuite,
    pre_test: PreTest,
    test_program: TestProgram,
    post_test: PostTest,
    post_suite: PostSuite,
}

impl RapidFuzzBuilder<(), (), (), (), (), (), ()> {
    pub fn new() -> RapidFuzzBuilder<(), (), (), (), (), (), ()> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite: (),
            pre_test: (),
            test_program: (),
            post_test: (),
            post_suite: (),
        }
    }
}

impl<Suite, Test, PreTest, TestProgram, PostTest, PostSuite>
    RapidFuzzBuilder<Suite, Test, (), PreTest, TestProgram, PostTest, PostSuite>
{
    pub fn with_pre_suite<PreSuite: PreSuiteTrait<Suite>>(
        self,
        pre_suite: PreSuite,
    ) -> RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, TestProgram, PostTest, PostSuite> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite,
            pre_test: self.pre_test,
            test_program: self.test_program,
            post_test: self.post_test,
            post_suite: self.post_suite,
        }
    }

    pub fn without_pre_suite(
        self,
    ) -> RapidFuzzBuilder<(), Test, NoOpSetup, PreTest, TestProgram, PostTest, PostSuite> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite: NoOpSetup,
            pre_test: self.pre_test,
            test_program: self.test_program,
            post_test: self.post_test,
            post_suite: self.post_suite,
        }
    }
}

impl<Suite, Test, PreSuite, TestProgram, PostTest, PostSuite>
    RapidFuzzBuilder<Suite, Test, PreSuite, (), TestProgram, PostTest, PostSuite>
{
    pub fn with_pre_test<PreTest: PreTestTrait<Suite, Test>>(
        self,
        pre_test: PreTest,
    ) -> RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, TestProgram, PostTest, PostSuite> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite: self.pre_suite,
            pre_test,
            test_program: self.test_program,
            post_test: self.post_test,
            post_suite: self.post_suite,
        }
    }

    pub fn without_pre_test(
        self,
    ) -> RapidFuzzBuilder<Suite, (), PreSuite, NoOpSetup, TestProgram, PostTest, PostSuite> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite: self.pre_suite,
            pre_test: NoOpSetup,
            test_program: self.test_program,
            post_test: self.post_test,
            post_suite: self.post_suite,
        }
    }
}

impl<Suite, Test, PreSuite, PreTest, PostTest, PostSuite>
    RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, (), PostTest, PostSuite>
{
    pub fn with_test_program<TestProgram: ProgramTrait<Suite, Test>>(
        self,
        test_program: TestProgram,
    ) -> RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, TestProgram, PostTest, PostSuite> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite: self.pre_suite,
            pre_test: self.pre_test,
            test_program,
            post_test: self.post_test,
            post_suite: self.post_suite,
        }
    }
}

impl<Suite, Test, PreSuite, PreTest, TestProgram, PostSuite>
    RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, TestProgram, (), PostSuite>
{
    pub fn with_post_test<PostTest: PostTestTrait<Suite, Test>>(
        self,
        post_test: PostTest,
    ) -> RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, TestProgram, PostTest, PostSuite> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite: self.pre_suite,
            pre_test: self.pre_test,
            test_program: self.test_program,
            post_test,
            post_suite: self.post_suite,
        }
    }

    pub fn without_post_test(
        self,
    ) -> RapidFuzzBuilder<Suite, (), PreSuite, PreTest, TestProgram, NoOpSetup, PostSuite> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite: self.pre_suite,
            pre_test: self.pre_test,
            test_program: self.test_program,
            post_test: NoOpSetup,
            post_suite: self.post_suite,
        }
    }
}

impl<Suite, Test, PreSuite, PreTest, TestProgram, PostTest>
    RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, TestProgram, PostTest, ()>
{
    pub fn with_post_suite<PostSuite: PostSuiteTrait<Suite>>(
        self,
        post_suite: PostSuite,
    ) -> RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, TestProgram, PostTest, PostSuite> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite: self.pre_suite,
            pre_test: self.pre_test,
            test_program: self.test_program,
            post_test: self.post_test,
            post_suite,
        }
    }

    pub fn without_post_suite(
        self,
    ) -> RapidFuzzBuilder<(), Test, PreSuite, PreTest, TestProgram, PostTest, NoOpSetup> {
        RapidFuzzBuilder {
            suite: PhantomData,
            test: PhantomData,
            pre_suite: self.pre_suite,
            pre_test: self.pre_test,
            test_program: self.test_program,
            post_test: self.post_test,
            post_suite: NoOpSetup,
        }
    }
}

impl<Suite, Test, PreSuite, PreTest, TestProgram, PostTest, PostSuite>
    RapidFuzzBuilder<Suite, Test, PreSuite, PreTest, TestProgram, PostTest, PostSuite>
where
    PreSuite: PreSuiteTrait<Suite>,
    PreTest: PreTestTrait<Suite, Test>,
    TestProgram: ProgramTrait<Suite, Test>,
    PostTest: PostTestTrait<Suite, Test>,
    PostSuite: PostSuiteTrait<Suite>,
{
    pub fn run(&self) {
        let args = parse_clap();
        let cases = self.derive_cases(&args);
        let suite = self.pre_suite.setup_suite();
        for (_iteration, (_seed, rng)) in cases.into_iter().enumerate() {
            let test_data = self.pre_test.setup_test(&suite);
            self.test_program.run(&suite, &test_data, rng);
            self.post_test.teardown_test(&suite, test_data);
        }
        self.post_suite.teardown_suite(suite);
    }

    fn derive_cases(&self, args: &DeterministicSimulationArgs) -> Vec<(SeedPhrase, ChaChaRng)> {
        match args.seed {
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
