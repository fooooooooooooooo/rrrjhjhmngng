use ariadne::{sources, Color, Label, Report, ReportKind};
use lexington::parser::parse;

const FILENAME: &str = "tests.css";

fn main() {
  let src = std::fs::read_to_string(FILENAME).unwrap();

  let (tokens, (tokenizing_errors, parser_errors)) = parse(&src);

  if let Some(tokens) = &tokens {
    for (name, key, format) in tokens {
      println!("name: {name:?}, key: {key:?}, format: {:?}, {:?}", format.0, format.1)
    }
  }

  tokenizing_errors.into_iter().chain(parser_errors).for_each(|e| {
    Report::build(ReportKind::Error, FILENAME, e.span().start)
      .with_message(e.to_string())
      .with_label(
        Label::new((FILENAME, e.span().into_range()))
          .with_message(e.reason().to_string())
          .with_color(Color::Red),
      )
      .with_labels(e.contexts().map(|(label, span)| {
        Label::new((FILENAME, span.into_range()))
          .with_message(format!("while parsing this {}", label))
          .with_color(Color::Yellow)
      }))
      .finish()
      .print(sources([(FILENAME, &src)]))
      .unwrap()
  });
}
