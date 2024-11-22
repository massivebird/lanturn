use clap::builder::PossibleValue;
use clap::{Arg, ArgMatches};

pub enum OutputFmt {
    Bullet,
    Line,
}

pub(super) fn generate_matches() -> ArgMatches {
    clap::command!()
        .arg(
            Arg::new("output_fmt")
                .long("output-fmt")
                .short('o')
                .value_parser([PossibleValue::new("bullet"), PossibleValue::new("line")])
                .help("Output format (default: \"bullet\")")
                .value_name("format")
                .value_hint(clap::ValueHint::Other)
                .required(false),
        )
        .get_matches()
}
