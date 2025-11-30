# ADR-0005: Validation Error Code Conventions

## Status
Accepted

## Context
SketchDDD performs validation on domain models to help users identify issues ranging from critical errors that must be fixed to suggestions for improvement. Users need a clear, consistent way to understand the severity of validation results and what action is required.

Non-technical users (domain experts, business analysts) will frequently use SketchDDD, so the error reporting system must be:
1. Immediately understandable without deep technical knowledge
2. Consistent and predictable
3. Actionable - users should know what to do

We needed to decide on a convention for error codes that communicates severity at a glance.

## Decision
We adopt a prefix-based error code convention:

### Prefixes
- **E** (Error): Issues that **must** be fixed. The model is invalid and cannot be processed further.
- **W** (Warning): Issues that **should** be reviewed. The model is valid but may have problems.
- **H** (Hint): Suggestions for improvement. Optional to address.

### Error Code Ranges
Error codes are organized by category:

| Range | Category |
|-------|----------|
| E0001-E0009 | Morphism reference errors |
| E0010-E0019 | Equation validation errors |
| E0020-E0029 | Duplicate name errors |
| E0030-E0039 | Aggregate structure errors |
| E0040-E0049 | Entity validation errors |
| E0050-E0059 | Enum/colimit errors |
| E0060-E0069 | Context map reference errors |
| E0070-E0079 | Model-level errors |
| W0001-W0009 | Aggregate warnings |
| W0010-W0019 | Value object warnings |

### Severity Levels
The `Severity` enum in code reflects this:
- `Severity::Error` - Validation fails, must fix
- `Severity::Warning` - Validation passes, review recommended
- `Severity::Hint` - Validation passes, optional improvement

### Validation Result Semantics
- `is_ok()` returns `true` if there are **no errors** (warnings are acceptable)
- `has_issues()` returns `true` if there are **any issues** (errors, warnings, or hints)

## Consequences

### Positive
- Users can immediately identify severity by looking at the prefix (E vs W vs H)
- Numeric ranges allow logical grouping of related issues
- Non-technical users can understand "E = must fix, W = should review"
- Error codes are stable and can be referenced in documentation
- Machine-readable for tooling integration

### Negative
- Must maintain documentation mapping codes to explanations
- New validation rules must fit into existing ranges or new ranges must be allocated
- Codes are less descriptive than full names (though messages provide details)

### Neutral
- Similar to conventions in rustc, TypeScript, and other mature tools
- Users familiar with compiler error conventions will feel at home

## References
- [Issue #7: Implement basic validation](https://github.com/ibrahimcesar/SketchDDD/issues/7)
- [Issue #35: Create comprehensive documentation site with MkDocs](https://github.com/ibrahimcesar/SketchDDD/issues/35)
- [Rust Compiler Error Index](https://doc.rust-lang.org/error_codes/error-index.html) - inspiration for error code conventions
