# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for SketchDDD.

## What is an ADR?

An Architecture Decision Record (ADR) is a document that captures an important architectural decision made along with its context and consequences.

## ADR Index

| ADR | Title | Status |
|-----|-------|--------|
| [0000](0000-adr-template.md) | ADR Template | Template |
| [0001](0001-use-category-theory-as-foundation.md) | Use Category Theory as Mathematical Foundation | Accepted |
| [0002](0002-rust-for-core-engine.md) | Use Rust for Core Engine | Accepted |
| [0003](0003-dual-interface-visual-and-text.md) | Dual Interface - Visual Builder and Text DSL | Accepted |
| [0004](0004-visual-builder-frontend-framework.md) | Visual Builder Frontend Framework | Proposed |
| [0005](0005-validation-error-code-conventions.md) | Validation Error Code Conventions | Accepted |
| [0006](0006-dsl-syntax-design.md) | DSL Syntax Design | Accepted |
| [0007](0007-file-extension-convention.md) | File Extension Convention (.sddd) | Accepted |
| [0008](0008-code-generation-architecture.md) | Code Generation Architecture | Accepted |
| [0009](0009-rich-error-messages.md) | Rich Error Messages | Accepted |
| [0010](0010-wasm-api-design.md) | WASM API Design | Accepted |
| [0011](0011-state-management-strategy.md) | State Management Strategy | Accepted |
| [0012](0012-template-format-specification.md) | Template Format Specification | Accepted |

## How to Create an ADR

1. Copy `0000-adr-template.md` to `NNNN-title-with-dashes.md`
2. Fill in all sections
3. Submit as part of your PR
4. Update this README with the new ADR

## ADR Lifecycle

- **Proposed**: Under discussion
- **Accepted**: Decision has been made
- **Deprecated**: No longer relevant
- **Superseded**: Replaced by another ADR
