extern crate prost_build;

fn main() {
  println!("cargo:rerun-if-changed=proto");
  prost_build::Config::new()
    .include_file("_includes.rs")
    .compile_protos(&["proto/kiririn.proto", "proto/kirino.proto"], &["proto"])
    .unwrap();
}
