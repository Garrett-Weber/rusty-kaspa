#![no_main]

use kaspa_consensus_core::hashing::sighash::SigHashReusedValues;
use kaspa_consensus_core::tx::PopulatedTransaction;
use kaspa_txscript::caches::Cache;
use kaspa_txscript::TxScriptEngine;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let sig_cache = Cache::new(100);
    let mut reused_values = SigHashReusedValues::new();
    let mut tx_engine: TxScriptEngine<PopulatedTransaction> = TxScriptEngine::from_script(data, &mut reused_values, &sig_cache);
    let a = tx_engine.execute();

    let sig_cache = Cache::new(100);
    let mut reused_values = SigHashReusedValues::new();
    let mut tx_engine: TxScriptEngine<PopulatedTransaction> = TxScriptEngine::from_script(data, &mut reused_values, &sig_cache);
    let b = tx_engine.execute();

    assert_eq!(a, b);
});
