#[macro_use]
extern crate rustler;

use rustler::{Encoder, Env, NifResult, Term};

rustler_export_nifs! {
    "Elixir.RustlerCoreDump",
    [
        ("roundtrip", 1, roundtrip),
    ],
    None
}

fn roundtrip<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let original: Term = args[0].decode()?;
    eprintln!("{:?}", original);
    let binary = original.to_binary();
    let roundtripped: Term = binary.to_term(env);
    eprintln!("{:?}", roundtripped);
    Ok((1, roundtripped).encode(env))
}
