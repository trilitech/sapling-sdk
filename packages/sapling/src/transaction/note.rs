use zcash_primitives::primitives::{Note, PaymentAddress, Rseed};

use crate::common::errors::{CausedBy, SaplingError};
use crate::transaction::errors::NoteError;

pub fn create_note(
    payment_address: &PaymentAddress,
    value: u64,
    rcm: jubjub::Scalar,
) -> Result<Note, SaplingError> {
    let rseed = Rseed::BeforeZip212(rcm);
    payment_address
        .create_note(value, rseed)
        .ok_or(NoteError::NoteEmpty)
        .map_err(SaplingError::caused_by)
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use ff::PrimeField;
    use hex;
    use zcash_primitives::primitives::PaymentAddress;

    use crate::common::traits::Serializable;

    use super::create_note;

    #[test]
    fn creates_note_commitments_from_octez_vectors() {
        // Authoritative source:
        // repos/tezos/src/lib_sapling/test/vectors.csv
        // That file documents its upstream lineage as:
        // https://github.com/zcash-hackworks/zcash-test-vectors
        let test_data = vec![
            (
                "f19d9b797e39f337445839db4cd2b0aac4f7eb8ca131f16567c445a9555126d3c29f14e3d776e841ae7415",
                0_u64,
                "39176dac39ace4980ecc8d778e89860255ec3615060000000000000000000000",
                "cb3cf9153270d57eb914c6c2bcc01850c9fed44fce0806278f083ef2dd076439",
            ),
            (
                "aef180f6e34e354b888f81a6b13ea336ddb7a67bb09a0e68e9d3cfb39210831ea3a296ba09a922060fd38b",
                395043257984320_u64,
                "478ba0ee6e1a75b600036f26f18b7015ab556beddf8b960238869f89dd804e06",
                "9255db54569746f13391c68ef872d8a84e33882fbf17de7da64b142f0b9d2069",
            ),
        ];

        for (address, value, rcm, expected_cmu) in test_data {
            let payment_address =
                PaymentAddress::deserialize(hex::decode(address).unwrap()).unwrap();
            let rcm =
                jubjub::Scalar::from_repr(hex::decode(rcm).unwrap().try_into().unwrap()).unwrap();
            let actual = create_note(&payment_address, value, rcm).unwrap().cmu();

            assert_eq!(
                actual.to_repr().as_ref(),
                hex::decode(expected_cmu).unwrap().as_slice()
            );
        }
    }
}
