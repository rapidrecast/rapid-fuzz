use arbitrary::{Arbitrary, Error, Unstructured};
use std::cmp::max;

pub struct OwnedArbitrary {
    buffer: Vec<u8>,
}

impl OwnedArbitrary {
    pub fn arbitrary<'a, Arb: Arbitrary<'a>>(&'a self) -> Arb {
        let mut unstructured = Unstructured::new(self.buffer.as_ref());
        match Arb::arbitrary(&mut unstructured) {
            Ok(a) => a,
            Err(_) => unreachable!(),
        }
    }
}

pub trait GenerateArbitrary<'data, A: Arbitrary<'data>> {
    fn generate_arbitrary<RNG: rand::Rng>(mut rng: &mut RNG) -> OwnedArbitrary {
        let depth = rng.random_range(0..10usize);
        let (hint_lower, _hint_upper) = A::size_hint(depth);
        let hint_lower = max(hint_lower, 1);
        let mut buffer = Vec::with_capacity(hint_lower);
        rng.fill_bytes(&mut buffer);
        // We are going to keep doubling the buffer until we get the type
        const MAX_TRIES: u8 = 20;
        for _ in 0..MAX_TRIES {
            let mut unstructured = Unstructured::new(&buffer);
            match A::arbitrary(&mut unstructured) {
                Ok(a) => return OwnedArbitrary { buffer },
                Err(Error::NotEnoughData) => {
                    // Double the buffer size
                    let mut next_data = Vec::with_capacity(buffer.len());
                    rng.fill_bytes(&mut next_data);
                    buffer.extend(next_data);
                }
                Err(Error::IncorrectFormat) | Err(Error::EmptyChoose) => {
                    panic!("Unexpected error")
                }
                Err(_) => unreachable!(),
            }
        }
        panic!("Absolutely massive arbitrary failed to generate")
    }
}

impl<'data, A: Arbitrary<'data>> GenerateArbitrary<'data, A> for A {}
