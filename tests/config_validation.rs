use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use tunex_tokenizer::{TokenizerConfig, ValidateTokenizerConfig};

#[test]
fn valid_tokenizer_config_passes_validation() {
    let path = fixture_path();
    fs::write(&path, "{}").expect("fixture");

    let config = TokenizerConfig {
        tokenizer_path: Some(path.clone()),
        max_seq_len: Some(4096),
        add_bos: true,
        add_eos: true,
        bos_token_id: Some(1),
        eos_token_id: Some(2),
        chat_template: Some("llama3".to_owned()),
        workers: 8,
        batch_size: 2048,
        ..TokenizerConfig::default()
    };

    let report = config.validate();

    assert!(report.is_valid(), "{:?}", report.errors);
    fs::remove_file(path).expect("cleanup");
}

#[test]
fn validation_rejects_missing_file_and_zero_limits() {
    let config = TokenizerConfig {
        tokenizer_path: Some(std::path::PathBuf::from(
            "/definitely/missing/tokenizer.json",
        )),
        max_seq_len: Some(0),
        workers: 0,
        batch_size: 0,
        ..TokenizerConfig::default()
    };

    let report = config.validate();

    assert!(report
        .errors
        .iter()
        .any(|error| error.path == "tokenizer_path"));
    assert!(report
        .errors
        .iter()
        .any(|error| error.path == "max_seq_len"));
    assert!(report.errors.iter().any(|error| error.path == "workers"));
    assert!(report.errors.iter().any(|error| error.path == "batch_size"));
}

#[test]
fn validation_rejects_unknown_template_and_missing_bos_eos_ids() {
    let config = TokenizerConfig {
        tokenizer_name: Some("hf/test".to_owned()),
        chat_template: Some("not-real".to_owned()),
        add_bos: true,
        add_eos: true,
        ..TokenizerConfig::default()
    };

    let report = config.validate();

    assert!(report
        .errors
        .iter()
        .any(|error| error.path == "chat_template"));
    assert!(report
        .errors
        .iter()
        .any(|error| error.path == "bos_token_id"));
    assert!(report
        .errors
        .iter()
        .any(|error| error.path == "eos_token_id"));
}

#[test]
fn validation_checks_padding_truncation_compatibility() {
    let config = TokenizerConfig {
        tokenizer_name: Some("hf/test".to_owned()),
        padding: true,
        truncate: true,
        pad_to_length: Some(128),
        max_seq_len: Some(256),
        ..TokenizerConfig::default()
    };

    let report = config.validate();

    assert!(report.errors.iter().any(|error| error.path == "padding"));
}

fn fixture_path() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "tunex-tokenizer-config-{}.json",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ))
}
