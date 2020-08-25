use std::env;
use std::path::Path;
use std::process::{Stdio, Command};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir);

    clean();
    setup();
    build();
    install(&dst);

    println!("cargo:rustc-link-search={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=termbox");
}

fn setup() {
    let mut cmd = Command::new("git");
    cmd.arg("clone");
    cmd.arg("https://github.com/nullgemm/termbox_next");
    cmd.arg(".termbox");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    cmd.current_dir(&cargo_dir);

    run(&mut cmd);
}

fn clean() {
    let mut cmd = Command::new("rm");
    cmd.arg("-rf");
    cmd.arg(".termbox");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_dir = Path::new(&manifest_dir);
    cmd.current_dir(&cargo_dir);
    run(&mut cmd);
}

fn build() {
    let mut cmd = Command::new("make");
    cmd.arg("-C");
    cmd.arg(".termbox");
    run(&mut cmd);
}

fn install(dst: &Path) {
    // my laziness in not using std::fs is astounding
    let mut mkdircmd = Command::new("mkdir");
    mkdircmd.arg("-p");
    mkdircmd.arg(format!("{}/lib/", dst.display()));
    run(&mut mkdircmd);

    let mut cmd = Command::new("install");
    cmd.arg("-D");
    cmd.arg(".termbox/bin/termbox.a");
    cmd.arg(format!("{}/lib/libtermbox.a", dst.display()));
    run(&mut cmd);
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success());
}
