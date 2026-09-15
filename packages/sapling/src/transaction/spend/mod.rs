#[cfg(any(feature = "c_bindings", feature = "wasm_bindings"))]
pub(crate) use description::{
    compute_nullifier, prepare_spend_description, sign_spend_description, UnsignedSpendDescription,
};
#[cfg(any(feature = "c_bindings", feature = "wasm_bindings"))]
pub(crate) use proof::{SpendDetails, SpendParameters};

mod description;
mod errors;
mod proof;
