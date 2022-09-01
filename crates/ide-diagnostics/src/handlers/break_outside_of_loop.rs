use crate::{Diagnostic, DiagnosticsContext};

// Diagnostic: break-outside-of-loop
//
// This diagnostic is triggered if the `break` keyword is used outside of a loop.
pub(crate) fn break_outside_of_loop(
    ctx: &DiagnosticsContext<'_>,
    d: &hir::BreakOutsideOfLoop,
) -> Diagnostic {
    let construct = if d.is_break { "break" } else { "continue" };
    Diagnostic::new(
        "break-outside-of-loop",
        format!("{construct} outside of loop"),
        ctx.sema.diagnostics_display_range(d.expr.clone().map(|it| it.into())).range,
    )
}

#[cfg(test)]
mod tests {
    use crate::tests::check_diagnostics;

    #[test]
    fn outside_of_loop() {
        check_diagnostics(
            r#"
fn foo() {
    break;
  //^^^^^ error: break outside of loop
    break 'a;
  //^^^^^^^^ error: break outside of loop
    continue;
  //^^^^^^^^ error: continue outside of loop
    continue 'a;
  //^^^^^^^^^^^ error: continue outside of loop
}
"#,
        );
    }
}
