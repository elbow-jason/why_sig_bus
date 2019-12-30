use rustler::{Env, Error, Term, Atom, NifUntaggedEnum, Decoder};

mod atoms {
    rustler::atoms! {
        ok,
        bad_arg,
    }
}

rustler::init! {
    "Elixir.WhySigBus",
    [
        decode_args
    ],
    load = load
}

fn load(_env: Env, _term: Term) -> bool {
    true
}

#[derive(NifUntaggedEnum, Debug, Clone)]
pub enum Arg {
    Str(String),
    Atom(Atom),
}

#[derive(Debug, Clone)]
pub enum Args {
    One(Arg),
    Zero
}

impl Decoder<'_> for Args {
    fn decode(term: Term) -> Result<Args, Error> {
        if let Ok(()) = term.decode() {
            return Ok(Args::Zero);
        }
        if let Ok(Args::One(arg)) = term.decode() {
            return Ok(Args::One(arg));
        }
        Err(Error::Term(Box::new(atoms::bad_arg())))
    }
}

#[rustler::nif]
pub fn decode_args(args: Args) -> Atom {
    println!("decode_args called with args: {:?}\r", args);
    atoms::ok()
}