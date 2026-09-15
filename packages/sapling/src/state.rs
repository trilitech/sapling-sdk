use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

use zcash_proofs::ZcashParameters;

use crate::common::errors::{CausedBy, SaplingError};

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);
static PROOF_PARAMS: OnceLock<ZcashParameters> = OnceLock::new();

pub struct State;

impl State {
    pub fn is_initialized() -> bool {
        IS_INITIALIZED.load(Ordering::Acquire)
    }

    pub fn set_initialized() {
        IS_INITIALIZED.store(true, Ordering::Release);
    }

    pub fn proof_params() -> Result<&'static ZcashParameters, SaplingError> {
        PROOF_PARAMS
            .get()
            .ok_or_else(|| SaplingError::caused_by("sapling parameters have not been initialized"))
    }

    pub fn set_proof_params(params: ZcashParameters) {
        let _ = PROOF_PARAMS.set(params);
    }
}
