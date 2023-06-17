use ariadne::{sources, Color, Label, Report, ReportKind};
use lexington::parser::{parse, Format, Target};

const FILENAME: &str = "tests.css";

fn main() {
  let src = std::fs::read_to_string(FILENAME).unwrap();

  let (targets, errors) = parse(&src);

  if let Some(targets) = targets {
    for (Target(name, format), span) in targets {
      let format = match format {
        Some(Format::Full(n, v)) => format!("{n: >5}({}, {}, {})", v.0, v.1, v.2),
        Some(Format::Name(n)) => format!("{n: >5}()"),
        Some(Format::Values(v)) => format!("     ({}, {}, {})", v.0, v.1, v.2),
        None => String::new(),
      };

      let source = format!("{:<42}", &src[span.into_range()]);
      let span = format!("{:<10}", span.to_string());
      let name = format!("{:<24}", name);

      println!("{span} = {source} | {name} {format}");
    }
  }

  errors.into_iter().for_each(|e| {
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
