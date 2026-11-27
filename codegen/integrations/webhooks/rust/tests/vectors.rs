//! Runs the shared conformance vectors against the Rust port.
//!
//! Reads the TSV rather than the JSON: this crate has no JSON dependency and
//! adding one just to run a test would be a second thing that can be wrong.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use crossly_webhooks::{verify, VerifyOptions};

#[test]
fn agrees_with_the_shared_vectors() {
    let raw = std::fs::read_to_string("../test-vectors.tsv").expect("vectors present");
    let mut ok = 0usize;
    let mut bad = 0usize;
    let mut total = 0usize;

    for line in raw.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 6 {
            continue;
        }
        let name = cols[0];
        let body = STANDARD.decode(cols[1]).expect("valid base64");
        let header = cols[2];
        let secret = cols[3];
        let now: i64 = cols[4].parse().expect("numeric now");
        let expect = cols[5];
        total += 1;

        let opts = VerifyOptions { tolerance_seconds: 300, now: Some(now) };
        let got = match verify(&body, Some(header), secret, opts) {
            Ok(_) => "ok",
            Err(e) => e.reason(),
        };

        if got == expect {
            ok += 1;
        } else {
            bad += 1;
            println!("  MISMATCH {name}: expected {expect}, got {got}");
        }
    }

    println!("  rust: {ok}/{total} vectors agree");
    assert_eq!(bad, 0, "{bad} vector(s) disagree");
}
