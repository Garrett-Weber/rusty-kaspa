#![no_main]

use kaspa_consensus_core::tx::PopulatedTransaction;
use kaspa_txscript::is_unspendable;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    assert_eq!(is_unspendable::<PopulatedTransaction>(data), is_unspendable::<PopulatedTransaction>(data));
});
