extern crate autocfg;

fn main() {
    let ac = autocfg::new();

    // autocfg doesn't have a direct way to probe for `const fn` yet.
    if ac.probe_rustc_version(1, 31) {
        autocfg::emit("has_const_fn");
    }

    autocfg::rerun_path("build.rs");
}
