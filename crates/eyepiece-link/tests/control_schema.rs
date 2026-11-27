//! contract/tinkerglass-control.proto is a copy of tinkerglass' own schema (its
//! contract/tinkerglass-control.proto). With a tinkerglass checkout next to this repository, or
//! TINKERGLASS_DIR pointing to one, the copy must be the same (line endings aside); without one
//! the test says so and passes.

use std::path::PathBuf;

#[test]
fn control_schema_matches_tinkerglass() {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let ours = here.join("contract/tinkerglass-control.proto");
    let tinkerglass = std::env::var_os("TINKERGLASS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| here.join("../tinkerglass"));
    let theirs = tinkerglass.join("contract/tinkerglass-control.proto");
    let read = |p: &PathBuf| std::fs::read_to_string(p).map(|s| s.replace("\r\n", "\n"));
    let ours = read(&ours).expect("our copy of the control schema");
    let Ok(theirs) = read(&theirs) else {
        eprintln!("no tinkerglass checkout at {} (set TINKERGLASS_DIR): not compared", tinkerglass.display());
        return;
    };
    assert!(
        ours == theirs,
        "contract/tinkerglass-control.proto differs from {}: copy tinkerglass' schema here",
        tinkerglass.display()
    );
}
