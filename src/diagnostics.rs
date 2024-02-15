/*
use ariadne::{Color, Label, Report, ReportKind};

fn generate_syntax_error<'src>() -> Report<'src> {
    let a = Color::Red;

    let rep = Report::build(ReportKind::Error, "test.drt", 3)
        .with_code(69)
        .with_message(format!("Syntax Error"))
        .with_label(
            Label::new(("test.drt", 16..22))
                .with_message(format!("Expected action start recieved {}", "iden"))
                .with_color(a),
        )
        .finish();
    rep
}
*/
