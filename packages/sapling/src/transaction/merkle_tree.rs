use zcash_primitives::merkle_tree::{Hashable, MerklePath};
use zcash_primitives::sapling::merkle_hash;

use crate::common::errors::{CausedBy, SaplingError};
use crate::common::traits::Serializable;
use crate::transaction::errors::MerklePathError;

impl<Node: Hashable> Serializable<Vec<u8>, SaplingError> for MerklePath<Node> {
    fn deserialize(serialized: Vec<u8>) -> Result<Self, SaplingError> {
        MerklePath::from_slice(&serialized[..])
            .map_err(|_| MerklePathError::ReadFailed)
            .map_err(SaplingError::caused_by)
    }

    fn serialize(&self) -> Result<Vec<u8>, SaplingError> {
        Err(SaplingError::caused_by(MerklePathError::CannotWrite))
    }
}

pub fn hash(depth: usize, lhs: [u8; 32], rhs: [u8; 32]) -> [u8; 32] {
    merkle_hash(depth, &lhs, &rhs)
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use hex;

    use super::hash;

    fn hex_bytes(hex_value: &str) -> [u8; 32] {
        hex::decode(hex_value).unwrap().try_into().unwrap()
    }

    fn reverse_hex_bytes(hex_value: &str) -> [u8; 32] {
        let mut bytes = hex_bytes(hex_value);
        bytes.reverse();
        bytes
    }

    fn octez_uncommitted_nodes(depth: usize) -> Vec<[u8; 32]> {
        let mut nodes = Vec::with_capacity(depth + 1);
        nodes.push(hex_bytes(
            "0100000000000000000000000000000000000000000000000000000000000000",
        ));

        for height in 0..depth {
            let node = hash(height, nodes[height], nodes[height]);
            nodes.push(node);
        }

        nodes
    }

    fn octez_commitments() -> Vec<[u8; 32]> {
        [
            "556f3af94225d46b1ef652abc9005dee873b2e245eef07fd5be587e0f21023b0",
            "5814b127a6c6b8f07ed03f0f6e2843ff04c9851ff824a4e5b4dad5b5f3475722",
            "6c030e6d7460f91668cc842ceb78cdb54470469e78cd59cf903d3a6e1aa03e7c",
            "30a0d08406b9e3693ee4c062bd1e6816f95bf14f5a13aafa1d57942c6c1d4250",
            "12fc3e7298eb327a88abcc406fbe595e45dddd9b4209803b2e0baa3a8663ecaa",
            "021a35cfe13d16891c1409d0f6e8865f51dd54792e5108a6f9e55e0dd44867f7",
            "2e0bfc1e123edcb6252251611650f3667371f781b60302385c414716c75e8abc",
            "11a5e54bf9a9b57e1c163904999ad1527f1e126c685111e18193decca2dd1ada",
            "4674f7836089063143fc18b673b2d92f888c63380e3680385d47bcdbd5fe273a",
            "0830165f36a69e416d51cc09cc5668692dee35d98539d3317999fdf87d8fcac7",
            "02372c746664e0898576972ca6d0500c7c8ec42f144622349d133b06e837faf0",
            "08c6d7dd3d2e387f7b84d6769f2b6cbe308918ab81e0f7321bd0945868d7d4e6",
            "26e8c4061f2ad984d19f2c0a4436b9800e529069c0b0d3186d4683e83bb7eb8c",
            "037cc2391338956026521beca5c81b541b7f2d1ead7758bf4d1588dbbcb8fa22",
            "1cc467cfd2b504e156c9a38bc5c0e4f5ea6cc208054d2d0653a7e561ac3a3ef4",
            "15ac4057a9a94536eca9802de65e985319e89627c9c64bc94626b712bc61363a",
        ]
        .iter()
        .map(|hex_value| reverse_hex_bytes(hex_value))
        .collect()
    }

    fn octez_expected_roots() -> Vec<[u8; 32]> {
        [
            "8c3daa300c9710bf24d2595536e7c80ff8d147faca726636d28e8683a0c27703",
            "8611f17378eb55e8c3c3f0a5f002e2b0a7ca39442fc928322b8072d1079c213d",
            "3db73b998d536be0e1c2ec124df8e0f383ae7b602968ff6a5276ca0695023c46",
            "7ac2e6442fec5970e116dfa4f2ee606f395366cafb1fa7dfd6c3de3ce18c4363",
            "6a8f11ab2a11c262e39ed4ea3825ae6c94739ccf94479cb69402c5722b034532",
            "149595eed0b54a7e694cc8a68372525b9ae2c7b102514f527460db91eb690565",
            "8c0432f1994a2381a7a4b5fda770336011f9e0b30784f9a5597901619c797045",
            "e780c48d70420601f3313ff8488d7766b70c059c53aa3cda2ff1ef57ff62383c",
            "f919f03caaed8a2c60f58c0d43838f83e670dc7e8ccd25daa04a13f3e8f45541",
            "74f32b36629724038e71cbd6823b5a666440205a7d1a9242e95870b53d81f34a",
            "a4af205a4e1ee02102866b23a68930ac33efda9235832f49b17fcc4939be4525",
            "a946a42f1636045a16e65b2308e036d9da70089686c87c692e45912bd1cab772",
            "a1db2dbac055364c1cb43cbeb49c7e2815bff855122602a2ad0fb981a91e0e39",
            "16329b3ba4f0640f4d306532d9ea6ba0fbf0e70e44ed57d27b4277ed9cda6849",
            "7b6523b2d9b23f72fec6234aa6a1f8fae3dba1c6a266023ea8b1826feba7a25c",
            "5c0bea7e17bde5bee4eb795c2eec3d389a68da587b36dd687b134826ecc09308",
        ]
        .iter()
        .map(|hex_value| hex_bytes(hex_value))
        .collect()
    }

    fn octez_root_from_leaves(depth: usize, leaves: &[[u8; 32]]) -> [u8; 32] {
        let defaults = octez_uncommitted_nodes(depth);
        let total_leaves = 1usize << depth;
        let mut level_nodes = vec![defaults[0]; total_leaves];

        for (index, leaf) in leaves.iter().enumerate() {
            level_nodes[index] = *leaf;
        }

        for height in 0..depth {
            level_nodes = level_nodes
                .chunks_exact(2)
                .map(|pair| hash(height, pair[0], pair[1]))
                .collect();
        }

        level_nodes[0]
    }

    #[test]
    fn hashes_merkle_nodes_using_official_zcash_vectors() {
        // Authoritative source:
        // https://raw.githubusercontent.com/zcash/zcash-test-vectors/master/zcash_test_vectors/sapling/merkle_tree.py
        //
        // The upstream vector literals are written in big-endian hex and reversed before the
        // bit-level Merkle CRH input is built. This crate, like zcash_primitives, operates on
        // the little-endian 32-byte node encodings directly, so the test mirrors that reversal.
        let lhs =
            reverse_hex_bytes("87a086ae7d2252d58729b30263fb7b66308bf94ef59a76c9c86e7ea016536505");
        let rhs =
            reverse_hex_bytes("a75b84a125b2353da7e8d96ee2a15efe4de23df9601b9d9564ba59de57130406");
        let expected =
            reverse_hex_bytes("5bf43b5736c19b714d1f462c9d22ba3492c36e3d9bbd7ca24d94b440550aa561");

        assert_eq!(hash(25, lhs, rhs), expected);
    }

    #[test]
    fn computes_octez_empty_tree_root() {
        // Authoritative source:
        // repos/tezos/src/lib_sapling/test/test_merkle.ml
        //
        // Octez starts from the Sapling uncommitted leaf `01 || 00*31` and hashes it with itself
        // from height 0 through 31. The expected root literal is stored big-endian there and
        // reversed before comparison.
        let mut root = octez_uncommitted_nodes(32)[0];

        for height in 0..32 {
            root = hash(height, root, root);
        }

        assert_eq!(
            root,
            reverse_hex_bytes("3e49b5f954aa9d3545bc6c37744661eea48d7c34e3000d82b7f0010c30f4c2fb")
        );
    }

    #[test]
    fn computes_octez_incremental_merkle_roots() {
        // Authoritative source:
        // repos/tezos/src/lib_sapling/test/test_merkle.ml
        let commitments = octez_commitments();
        let expected_roots = octez_expected_roots();

        for (index, expected_root) in expected_roots.iter().enumerate() {
            let actual_root = octez_root_from_leaves(4, &commitments[..=index]);
            assert_eq!(actual_root, *expected_root);
        }
    }
}
