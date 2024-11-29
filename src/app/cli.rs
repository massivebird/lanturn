use crate::app::OutputFmt;
use clap::{builder::EnumValueParser, Arg, ArgMatches, ValueEnum, ValueHint};

pub(super) fn generate_matches() -> ArgMatches {
    clap::command!()
        .arg(
            Arg::new("output_fmt")
                .long("output-fmt")
                .short('o')
                .value_parser(EnumValueParser::<OutputFmt>::new())
                .help(format!(
                    // I can't figure out how to dynamically use clap's
                    // `.default_value()` with `OutputFmt::default()`,
                    // so you get this instead.
                    "Output format [default: {}]",
                    OutputFmt::default().to_possible_value().unwrap().get_name()
                ))
                .value_name("format")
                .value_hint(ValueHint::Other)
                .required(false),
        )
        .get_matches()
}
