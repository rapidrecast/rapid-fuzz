use arbitrary::{Arbitrary, Error, Unstructured};
use std::cmp::max;

pub struct OwnedArbitrary {
    buffer: Vec<u8>,
}

impl OwnedArbitrary {
    pub fn arbitrary<Arb: for<'a> Arbitrary<'a>>(&self) -> arbitrary::Result<Arb> {
        let mut unstructured = Unstructured::new(self.buffer.as_ref());
        Arb::arbitrary(&mut unstructured)
    }
}

pub trait GenerateArbitrary<A: for<'a> Arbitrary<'a>> {
    fn generate_owned_arbitrary_data<RNG: rand::Rng>(rng: &mut RNG) -> OwnedArbitrary {
        let depth = rng.random_range(0..10usize);
        let (hint_lower, hint_upper) = A::size_hint(depth);
        let hint_lower = max(hint_lower, 1);
        let capacity = (hint_upper.unwrap_or(hint_lower) + 1) * 2;
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize(capacity, 0);
        rng.fill_bytes(&mut buffer);
        // We are going to keep doubling the buffer until we get the type
        const MAX_TRIES: u8 = 20;
        for _ in 0..MAX_TRIES {
            let owned_arbitrary = OwnedArbitrary {
                buffer: buffer.clone(),
            };
            match owned_arbitrary.arbitrary::<A>() {
                Ok(_) => {
                    return owned_arbitrary;
                }
                Err(Error::NotEnoughData) => {
                    // Double the buffer size
                    let mut next_data = Vec::with_capacity(buffer.len());
                    next_data.resize(next_data.capacity(), 0);
                    rng.fill_bytes(&mut next_data);
                    buffer.extend(next_data);
                }
                Err(Error::IncorrectFormat) | Err(Error::EmptyChoose) => {
                    panic!("Unexpected error, either incorrect format or empty choose")
                }
                Err(_) => unreachable!(),
            }
        }
        panic!("Absolutely massive arbitrary failed to generate")
    }
}

impl<A: for<'a> Arbitrary<'a>> GenerateArbitrary<A> for A {}
