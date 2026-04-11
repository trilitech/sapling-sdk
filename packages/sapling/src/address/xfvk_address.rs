use zcash_primitives::zip32::{DiversifierIndex, ExtendedFullViewingKey};

use crate::address::indexed_address::IndexedAddress;
use crate::common::errors::{CausedBy, SaplingError};

use super::errors::SaplingAddressError;

pub fn get_xfvk_address(
    xfvk: &ExtendedFullViewingKey,
    index: Option<[u8; 11]>,
) -> Result<IndexedAddress, SaplingError> {
    let (index, payment_address) = match index {
        Some(index) => xfvk.address(DiversifierIndex(index)),
        None => xfvk.default_address(),
    }
    .map_err(|_| SaplingAddressError::DiversifierSpaceExhausted)
    .map_err(SaplingError::caused_by)?;

    Ok(IndexedAddress::new(index, payment_address))
}

pub fn get_next_xfvk_address(
    xfvk: &ExtendedFullViewingKey,
    index: [u8; 11],
) -> Result<IndexedAddress, SaplingError> {
    let mut index = DiversifierIndex(index);
    index
        .increment()
        .map_err(|_| SaplingAddressError::DiversifierSpaceExhausted)
        .map_err(SaplingError::caused_by)?;

    get_xfvk_address(xfvk, Some(index.0))
}

#[cfg(test)]
mod tests {
    use crate::common::traits::Serializable;

    use crate::key::SaplingKey;

    use super::*;

    const SEED: [u8; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];

    #[test]
    fn gets_address_from_extended_full_viewing_key() {
        // Authoritative source:
        // https://raw.githubusercontent.com/zcash/zcash-test-vectors/master/test-vectors/json/sapling_zip32.json
        // Cross-checked against the C++ ZIP32 regression in:
        // https://raw.githubusercontent.com/zcash/zcash/master/src/gtest/test_zip32.cpp
        let test_data: Vec<(&str, Option<[u8; 11]>, [u8; 11], &str, &str)> = vec![
            ("m/", None, [0; 11], "d8621b981cf300e9d4cc89", "0000000000000000000000d8621b981cf300e9d4cc89c9caf24d58de249f97323c53f179b761979a470d003cd355d34a34272b824402"),
            ("m/1", None, [0; 11], "8b4138320dfafd7b399781", "00000000000000000000008b4138320dfafd7b399781a8cf24c3178536869042d734d23cf281fdfd4aca1df9060270420c49775668dd"),
            ("m/1/2h", None, [0; 11], "e8d03793cdd2bacc9c7041", "0000000000000000000000e8d03793cdd2bacc9c7041ad5ec1877b8ca3ada20125535e840498712bda116dbc506edaf52d94fb8c72de"),
            ("m/1/2h/3", None, [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "030ffb263a939e230e96dd", "0100000000000000000000030ffb263a939e230e96dd0805ba6dbe98d91f30f3b1ac40a8bca48ce1304da1da1012f81415dd7061c5f1"),
        ];

        let actual_expected = test_data.iter().map(
            |(path, i, expected_index, expected_diversifier, expected_serialized)| {
                let viewing_key = ExtendedFullViewingKey::from_seed(&SEED, path).unwrap();
                let actual = get_xfvk_address(&viewing_key, *i).unwrap();

                (
                    actual,
                    *expected_index,
                    hex::decode(expected_diversifier).unwrap(),
                    hex::decode(expected_serialized).unwrap(),
                )
            },
        );

        for (actual, expected_index, expected_diversifier, expected_serialized) in actual_expected {
            assert_eq!(actual.0, expected_index);
            assert_eq!(actual.1.diversifier().0, expected_diversifier.as_slice());
            assert_eq!(actual.serialize().unwrap(), expected_serialized);
        }
    }

    #[test]
    fn gets_next_address_from_extended_full_viewing_key() {
        // Authoritative source:
        // https://raw.githubusercontent.com/zcash/zcash-test-vectors/master/test-vectors/json/sapling_zip32.json
        let test_data: Vec<([u8; 11], &str, [u8; 11], &str)> = vec![
            ([0; 11], "m/", [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "010000000000000000000048ea17a199c84bd1baa5d4c64cb9b7744e42030875e7c22815bffd8c83c4e805927389f12d3c165497b093"),
            ([0; 11], "m/1/2h", [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "0100000000000000000000020a7a6b0bf84d3e899f68376c89212336c7a6e7fa7552326126805d98310a9cbbb3cd030ccf4412de3b89"),
        ];

        let actual_expected =
            test_data
                .iter()
                .map(|(i, path, expected_index, expected_serialized)| {
                    let viewing_key = ExtendedFullViewingKey::from_seed(&SEED, path).unwrap();
                    let actual = get_next_xfvk_address(&viewing_key, *i).unwrap();

                    (
                        actual,
                        *expected_index,
                        hex::decode(expected_serialized).unwrap(),
                    )
                });

        for (actual, expected_index, expected_serialized) in actual_expected {
            assert_eq!(actual.0, expected_index);
            assert_eq!(actual.serialize().unwrap(), expected_serialized);
        }
    }

    #[test]
    fn fails_to_get_next_address_from_extended_full_viewing_key_on_diversifier_index_overflow() {
        let max_index: [u8; 11] = [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255];
        let xfvk = ExtendedFullViewingKey::from_seed(&SEED, "m/").unwrap();

        let error = get_next_xfvk_address(&xfvk, max_index).unwrap_err();

        assert_eq!(
            error,
            SaplingError::caused_by(SaplingAddressError::DiversifierSpaceExhausted)
        )
    }
}
