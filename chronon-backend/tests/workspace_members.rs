//! Gate: chronon-app / chronon-backend / e2e lab / protected host are members of this workspace.
//!
//! Featureless sibling-source contract (photon / boson / gauge / lepton-shell pattern).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn chronon_product_workspace_members_happy_path() {
    let root =
        fs::read_to_string(workspace_root().join("Cargo.toml")).expect("workspace Cargo.toml");
    for member in [
        "chronon-app",
        "chronon-backend",
        "chronon-uf-app-e2e",
        "examples/protected-chronon-host",
    ] {
        assert!(
            root.contains(&format!("\"{member}\"")),
            "workspace must list {member}"
        );
        assert!(
            workspace_root().join(member).join("Cargo.toml").is_file(),
            "missing crate dir {member}"
        );
    }
}
