use clap::builder::{EnumValueParser, PossibleValue};
use clap::{Arg, ArgMatches, ValueEnum, ValueHint};

#[derive(Copy, Clone)]
pub enum OutputFmt {
    Bullet,
    Line,
}

impl ValueEnum for OutputFmt {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Bullet, Self::Line]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Bullet => Some(PossibleValue::new("bullet")),
            Self::Line => Some(PossibleValue::new("line")),
        }
    }
}

pub(super) fn generate_matches() -> ArgMatches {
    clap::command!()
        .arg(
            Arg::new("output_fmt")
                .long("output-fmt")
                .short('o')
                .value_parser(EnumValueParser::<OutputFmt>::new())
                .help("Output format (default: \"bullet\")")
                .default_value("bullet")
                .value_name("format")
                .value_hint(ValueHint::Other)
                .required(false),
        )
        .get_matches()
}
