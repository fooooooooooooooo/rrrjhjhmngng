use chumsky::prelude::*;

#[derive(Clone, Debug)]
pub struct Target<'a>(pub &'a str, pub Option<Format<'a>>);

#[derive(Clone, Debug)]
pub enum Format<'a> {
  Full(&'a str, (f32, f32, f32)),
  Name(&'a str),
  Values((f32, f32, f32)),
}

type ParseResult<'a> = Vec<(Target<'a>, SimpleSpan)>;

pub fn parse(input: &str) -> (Option<ParseResult>, Vec<Rich<'_, char>>) {
  let (colors, parse_errors) = parser().parse(input).into_output_errors();

  (colors, parse_errors)
}

fn parser<'i>() -> impl Parser<'i, &'i str, ParseResult<'i>, extra::Err<Rich<'i, char>>> {
  let ident = text::ident().padded();
  let digits = text::digits(10).slice();
  let frac = just('.').then(digits);

  let number = just('-')
    .or_not()
    .then(text::int(10))
    .then(frac.or_not())
    .map_slice(|s: &str| s.parse().unwrap())
    .boxed();

  let format_values = number
    .padded()
    .repeated()
    .at_least(3)
    .at_most(3)
    .collect_exactly::<[f32; 3]>()
    .map(|v| (v[0], v[1], v[2]))
    .delimited_by(just("("), just(")"));

  let format = just(".")
    .or_not()
    .ignore_then(ident.or_not())
    .then(format_values.or_not())
    .map(|format| match format {
      (Some(name), Some(values)) => Some(Format::Full(name, values)),
      (None, None) => None,
      (None, Some(values)) => Some(Format::Values(values)),
      (Some(name), None) => Some(Format::Name(name)),
    });

  let target = ident
    .then(format)
    .map(|(name, format)| Target(name, format))
    .delimited_by(just("@{"), just("}"));

  let not_start_sequence = any().and_is(just("@{").not()).repeated();

  not_start_sequence
    .ignore_then(target.map_with_span(|a: Target<'_>, b| (a, b)))
    .then_ignore(not_start_sequence)
    .repeated()
    .collect()
}
