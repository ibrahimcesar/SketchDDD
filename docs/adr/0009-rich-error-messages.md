# ADR-0009: Rich Error Messages

## Status
Accepted

## Context
SketchDDD validation produces error messages that users must understand and act upon. The target audience includes:

1. **Domain experts** - Non-programmers who may not understand technical jargon
2. **Developers** - Who expect compiler-quality error messages
3. **Tool integrations** - IDEs, CI systems that parse error output

Basic error messages like "Invalid morphism" are insufficient. Users need:
- Exact location of the problem
- Visual context (code snippet)
- Explanation of what's wrong
- Suggestions for how to fix it

Modern compilers (Rust, Elm, TypeScript) have set high standards for error message quality.

## Decision

### Error Message Structure

Every error message follows this structure:

```
error[E0023]: Unknown object referenced in morphism
  --> commerce.sddd:15:12
   |
14 |   morphisms {
15 |     placedBy: Order -> Custommer
   |                        ^^^^^^^^^ not found in context 'Commerce'
16 |   }
   |
   = help: did you mean `Customer`?
   = note: available objects: Customer, Order, Product, Money
```

### Components

1. **Severity and Code**: `error[E0023]` or `warning[W0001]`
2. **Summary**: Brief description of the problem
3. **Location**: `file:line:column` format
4. **Source Context**: Code snippet with line numbers
5. **Underline**: Visual marker pointing to the exact problem
6. **Primary Label**: Explanation on the underlined region
7. **Help**: Actionable suggestion (optional)
8. **Note**: Additional context (optional)

### Error Code Reference

Error codes follow ADR-0005 conventions and link to documentation:

```
For more information about this error, see:
  https://docs.sketchddd.dev/errors/E0023
```

### Visual Styling (Terminal)

| Element | Color | Style |
|---------|-------|-------|
| `error` | Red | Bold |
| `warning` | Yellow | Bold |
| `hint` | Blue | Bold |
| Error code | Red/Yellow | Normal |
| File path | White | Bold |
| Line numbers | Blue | Dim |
| Source code | White | Normal |
| Underline | Red/Yellow | Bold |
| `help:` | Cyan | Normal |
| `note:` | White | Dim |

### Multi-Span Errors

For errors involving multiple locations:

```
error[E0062]: Object mapping references non-existent source
  --> commerce.sddd:42:5
   |
42 |     Order -> Shipment
   |     ^^^^^ not found in source context
   |
  ::: commerce.sddd:8:1
   |
 8 | context Commerce {
   | ----------------- source context defined here
   |
   = help: add `Order` to the Commerce context
```

### "Did You Mean?" Suggestions

Use Levenshtein distance to suggest similar names:

```rust
fn suggest_similar(name: &str, candidates: &[&str], max_distance: usize) -> Option<&str> {
    candidates.iter()
        .filter_map(|c| {
            let dist = levenshtein(name, c);
            if dist <= max_distance { Some((c, dist)) } else { None }
        })
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| *c)
}
```

Threshold: suggest if edit distance ≤ 3 or ≤ 30% of name length.

### Error Grouping

Related errors are grouped to reduce noise:

```
error[E0001]: Multiple morphisms reference unknown objects
  --> commerce.sddd
   |
   = 3 morphisms reference `Custommer` (did you mean `Customer`?)
   |
15 |     placedBy: Order -> Custommer
   |                        ^^^^^^^^^
18 |     billedTo: Order -> Custommer
   |                        ^^^^^^^^^
22 |     shipsTo: Order -> Custommer
   |                       ^^^^^^^^^
```

### Machine-Readable Output

JSON format for tool integration (`--format json`):

```json
{
  "errors": [
    {
      "code": "E0023",
      "severity": "error",
      "message": "Unknown object referenced in morphism",
      "location": {
        "file": "commerce.sddd",
        "line": 15,
        "column": 12,
        "length": 9
      },
      "labels": [
        {
          "span": { "start": 245, "end": 254 },
          "message": "not found in context 'Commerce'"
        }
      ],
      "suggestions": [
        {
          "message": "did you mean `Customer`?",
          "replacement": "Customer"
        }
      ]
    }
  ],
  "warnings": [],
  "summary": {
    "errors": 1,
    "warnings": 0
  }
}
```

### Implementation Approach

Use the `ariadne` or `miette` crate for rendering:

```rust
use ariadne::{Report, ReportKind, Label, Source, Color};

Report::build(ReportKind::Error, file, offset)
    .with_code("E0023")
    .with_message("Unknown object referenced in morphism")
    .with_label(
        Label::new((file, span))
            .with_message("not found in context")
            .with_color(Color::Red)
    )
    .with_help("did you mean `Customer`?")
    .finish()
    .print((file, Source::from(source)))?;
```

## Consequences

### Positive
- Users can quickly locate and understand problems
- "Did you mean?" reduces frustration from typos
- Machine-readable output enables IDE integration
- Consistent format across all error types
- Documentation links provide deeper explanations

### Negative
- More complex error reporting code
- Need to track source spans throughout parsing
- Suggestion algorithm adds computational overhead
- Must maintain error documentation website

### Neutral
- Follows conventions established by modern compilers
- Requires terminal with color support for best experience
- Plain text fallback available for basic terminals

## References
- [Issue #12: Implement rich error messages](https://github.com/ibrahimcesar/SketchDDD/issues/12)
- [ADR-0005: Validation Error Code Conventions](0005-validation-error-code-conventions.md)
- [Rust Compiler Error Format](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_errors/index.html)
- [ariadne](https://github.com/zesterer/ariadne) - Rust diagnostic rendering
- [miette](https://github.com/zkat/miette) - Rust error reporting
