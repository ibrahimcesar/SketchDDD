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

## Proposed ADRs (To Be Written)

The following decisions need ADRs to be written during development:

| Proposed ADR | Related Issues | Phase |
|--------------|----------------|-------|
| DSL Syntax Design | #5, #6 | Phase 1 |
| Error Message Strategy | #7, #12 | Phase 1-2 |
| WASM API Design | #13 | Phase 3 |
| State Management Strategy | #14, #22 | Phase 3-4 |
| Code Generation Architecture | #23, #24, #25 | Phase 5 |
| Template Format Specification | #19 | Phase 4 |

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
