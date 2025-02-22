use crate::seed_phrase::SeedPhrase;
use clap::{Arg, Command};
use rand::{RngCore, SeedableRng};
use rand_chacha::{ChaCha20Rng, ChaChaRng};
use std::any::Any;
use std::marker::PhantomData;
use std::panic;

pub trait PreSuiteTrait<Suite> {
    fn setup_suite(&self) -> Suite;
}

pub trait PreTestTrait<Suite, TestData> {
    fn setup_test(&self, suite: &Suite, rng: &mut ChaCha20Rng) -> TestData;
}

pub trait PostTestTrait<Suite, TestData> {
    fn teardown_test(&self, suite: &Suite, test: TestData);
}

pub trait PostSuiteTrait<Suite> {
    fn teardown_suite(&self, suite: Suite);
}

pub trait ProgramTrait<Suite, TestData> {
    fn run<RNG: RngCore>(&self, suite: &Suite, test: &TestData, rng: RNG);
}

pub struct NoOpSetup;

impl PreSuiteTrait<()> for NoOpSetup {
    fn setup_suite(&self) -> () {}
}
impl<Suite> PreTestTrait<Suite, ()> for NoOpSetup {
    fn setup_test(&self, _suite: &Suite, _rng: &mut ChaCha20Rng) -> () {}
}

impl<Suite, TestData> PostTestTrait<Suite, TestData> for NoOpSetup {
    fn teardown_test(&self, _suite: &Suite, _test: TestData) {}
}
impl<Suite> PostSuiteTrait<Suite> for NoOpSetup {
    fn teardown_suite(&self, _suite: Suite) {}
}

pub struct RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest, PostSuite> {
    suite: PhantomData<Suite>,
    test: PhantomData<TestData>,
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

impl<Suite, TestData, PreTest, TestProgram, PostTest, PostSuite>
    RapidFuzzBuilder<Suite, TestData, (), PreTest, TestProgram, PostTest, PostSuite>
{
    pub fn with_pre_suite<PreSuite: PreSuiteTrait<Suite>>(
        self,
        pre_suite: PreSuite,
    ) -> RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest, PostSuite>
    {
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
    ) -> RapidFuzzBuilder<(), TestData, NoOpSetup, PreTest, TestProgram, PostTest, PostSuite> {
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

impl<Suite, PreSuite, TestProgram, PostTest, PostSuite>
    RapidFuzzBuilder<Suite, (), PreSuite, (), TestProgram, PostTest, PostSuite>
{
    pub fn with_pre_test<TestData, PreTest: PreTestTrait<Suite, TestData>>(
        self,
        pre_test: PreTest,
    ) -> RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest, PostSuite>
    {
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

impl<Suite, TestData, PreSuite, PreTest, PostTest, PostSuite>
    RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, (), PostTest, PostSuite>
{
    pub fn with_test_program<TestProgram: ProgramTrait<Suite, TestData>>(
        self,
        test_program: TestProgram,
    ) -> RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest, PostSuite>
    {
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

impl<Suite, TestData, PreSuite, PreTest, TestProgram, PostSuite>
    RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, (), PostSuite>
{
    pub fn with_post_test<PostTest: PostTestTrait<Suite, TestData>>(
        self,
        post_test: PostTest,
    ) -> RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest, PostSuite>
    {
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
    ) -> RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, NoOpSetup, PostSuite>
    {
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

impl<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest>
    RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest, ()>
{
    pub fn with_post_suite<PostSuite: PostSuiteTrait<Suite>>(
        self,
        post_suite: PostSuite,
    ) -> RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest, PostSuite>
    {
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
    ) -> RapidFuzzBuilder<(), TestData, PreSuite, PreTest, TestProgram, PostTest, NoOpSetup> {
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

impl<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest, PostSuite>
    RapidFuzzBuilder<Suite, TestData, PreSuite, PreTest, TestProgram, PostTest, PostSuite>
where
    PreSuite: PreSuiteTrait<Suite>,
    PreTest: PreTestTrait<Suite, TestData>,
    TestProgram: ProgramTrait<Suite, TestData>,
    PostTest: PostTestTrait<Suite, TestData>,
    PostSuite: PostSuiteTrait<Suite>,
{
    pub fn run(&self) {
        let args = parse_clap();
        let cases = self.derive_cases(&args);
        let suite = self.pre_suite.setup_suite();
        let mut an_error = None;
        let mut max_iteration = 0;
        for (iteration, (seed, rng)) in cases.into_iter().enumerate() {
            if let Err(e) = self.run_test_safely(&suite, seed, rng) {
                an_error = Some(e);
                break;
            }
            max_iteration = iteration;
        }
        self.post_suite.teardown_suite(suite);
        max_iteration += 1;
        if let Some(e) = an_error {
            eprintln!("Test failed after {} iterations", max_iteration);
            panic::resume_unwind(e);
        } else {
            println!("Test passed after {} iterations", max_iteration);
        }
    }

    fn run_test_safely(
        &self,
        suite: &Suite,
        seed: SeedPhrase,
        mut rng: ChaCha20Rng,
    ) -> Result<(), Box<dyn Any + Send>> {
        let test_data = self.pre_test.setup_test(&suite, &mut rng);
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            self.test_program.run(&suite, &test_data, rng);
        }));
        if let Err(_) = result {
            eprintln!("Test panicked for seed: {}", seed.to_string());
        }
        self.post_test.teardown_test(&suite, test_data);
        if let Err(e) = result {
            return Err(e);
        }
        Ok(())
    }

    fn derive_cases(&self, args: &DeterministicSimulationArgs) -> Vec<(SeedPhrase, ChaChaRng)> {
        match args.seed {
            Some(seed) => (0..=0)
                .map(|_| (seed, ChaCha20Rng::from_seed(seed.into_bytes())))
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
                            ChaCha20Rng::from_seed(seed),
                        )
                    })
                    .collect::<Vec<_>>()
            }
        }
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
                .help("Maximum number of iterations"),
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
