#[macro_use]
extern crate rustler;

use rustler::{NifEncoder, NifEnv, NifResult, NifTerm};

rustler_export_nifs! {
    "Elixir.RustlerCoreDump",
    [
        ("roundtrip", 1, roundtrip),
    ],
    None
}

fn roundtrip<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {
    let original: NifTerm = args[0].decode()?;
    let binary = original.to_binary();
    let roundtripped: NifTerm = binary.to_term(env);
    Ok((1, roundtripped).encode(env))
}
