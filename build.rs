use saphire_typegen::gen::gen_types;

fn main() -> () {
    gen_types("samples/db.json", "src/block/block_types");
}