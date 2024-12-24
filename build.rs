fn main() {
    cc::Build::new().file("src/gencdfa1.c").compile("gencdfa1");
    println!("cargo:rerun-if-changed=src/gencdfa1.c")
}
