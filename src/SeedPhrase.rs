/// SeedPhrase is a struct that allows human-readable representations of seed data
#[derive(Clone, Copy)]
pub struct SeedPhrase {
    data: [u8; 32],
}

impl SeedPhrase {
    pub fn parse(data: &str) -> Result<SeedPhrase, &'static str> {
        let data = data
            .as_bytes()
            .into_iter()
            .filter(|b| "0123456789abcdefABCDEF".as_bytes().contains(b))
            .map(|b| *b)
            .collect::<Vec<u8>>();
        if data.len() != 64 {
            return Err("Seed data must be 64 hex digits");
        }
        let input = data
            .as_slice()
            .chunks_exact(2)
            .map(|chunk| {
                let hex = std::str::from_utf8(chunk).unwrap();
                u8::from_str_radix(hex, 16).map_err(|_| "Invalid hex")
            })
            .collect::<Result<Vec<u8>, &'static str>>()?;
        let mut data = [0u8; 32];
        data.copy_from_slice(&input);
        Ok(SeedPhrase { data })
    }

    pub fn to_string(&self) -> String {
        self.data
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn from_bytes<B: AsRef<[u8]>>(bytes: &B) -> Result<SeedPhrase, &'static str> {
        let bytes = bytes.as_ref();
        if bytes.len() != 32 {
            return Err("Seed data must be 32 bytes");
        }
        let mut data = [0u8; 32];
        data.copy_from_slice(bytes);
        Ok(SeedPhrase { data })
    }

    pub fn into_bytes(self) -> [u8; 32] {
        self.data
    }
}

#[cfg(test)]
mod test {
    use crate::SeedPhrase::SeedPhrase;

    #[test]
    fn test_parse() {
        let seed = SeedPhrase::parse(
            "deadbeef-deadbeef-deadbeef-deadbeef-deadbeef-deadbeef-deadbeef-deadbeef",
        )
        .unwrap();
        let deadbeef_8 = [
            0xde, 0xad, 0xbe, 0xef, //
            0xde, 0xad, 0xbe, 0xef, //
            0xde, 0xad, 0xbe, 0xef, //
            0xde, 0xad, 0xbe, 0xef, //
            0xde, 0xad, 0xbe, 0xef, //
            0xde, 0xad, 0xbe, 0xef, //
            0xde, 0xad, 0xbe, 0xef, //
            0xde, 0xad, 0xbe, 0xef, //
        ];
        assert_eq!(
            &seed.data,
            &deadbeef_8,
            "{}\n{}",
            seed.to_string(),
            deadbeef_8.map(|b| format!("{:02x}", b)).join(""),
        );
        let seed = SeedPhrase::from_bytes(&deadbeef_8).unwrap();
        assert_eq!(&seed.data, &deadbeef_8);
    }
}
