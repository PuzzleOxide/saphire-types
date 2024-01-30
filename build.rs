use saphire_typegen::gen::gen_types;

fn main() -> () {
    gen_types("samples/db.json", "src/block/block_types");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=samples/db.json");
}