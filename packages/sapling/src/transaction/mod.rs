#[cfg(any(feature = "c_bindings", feature = "wasm_bindings"))]
pub(crate) use merkle_tree::hash as merkle_hash;
#[cfg(any(feature = "c_bindings", feature = "wasm_bindings"))]
pub(crate) use note::create_note;
#[cfg(any(feature = "c_bindings", feature = "wasm_bindings"))]
pub(crate) use output::{
    derive_epk, prepare_output_description, prepare_partial_output_description, OutputDetails,
};
#[cfg(any(feature = "c_bindings", feature = "wasm_bindings"))]
pub(crate) use proof::prepare_proof_parameters;
#[cfg(any(feature = "c_bindings", feature = "wasm_bindings"))]
pub(crate) use rand::rand_scalar;
#[cfg(any(feature = "c_bindings", feature = "wasm_bindings"))]
pub(crate) use signature::create_binding_sig;
#[cfg(any(feature = "c_bindings", feature = "wasm_bindings"))]
pub(crate) use spend::{
    compute_nullifier, prepare_spend_description, sign_spend_description, SpendDetails,
    SpendParameters, UnsignedSpendDescription,
};

mod output;
mod signature;
mod spend;

mod merkle_tree;
mod note;
mod proof;
mod rand;

mod errors;
