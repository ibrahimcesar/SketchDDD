# Contributing to SketchDDD

Thank you for your interest in contributing to SketchDDD! This document provides guidelines and information for contributors.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Pull Request Process](#pull-request-process)
- [Architecture Decision Records](#architecture-decision-records)
- [Style Guidelines](#style-guidelines)

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the maintainers.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/SketchDDD.git`
3. Add the upstream remote: `git remote add upstream https://github.com/ibrahimcesar/SketchDDD.git`
4. Create a branch for your changes: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites

- **Rust** (latest stable): https://rustup.rs/
- **Node.js** (20+): https://nodejs.org/
- **wasm-pack**: `cargo install wasm-pack`

### Building the Project

```bash
# Build Rust crates
cargo build

# Build WASM bindings
wasm-pack build crates/sketchddd-wasm --target web

# Install web dependencies
cd web && npm install

# Run web dev server
npm run dev
```

### Running Tests

```bash
# Rust tests
cargo test

# Web tests
cd web && npm test
```

## Project Structure

```
sketchddd/
├── crates/                    # Rust crates
│   ├── sketchddd-core/       # Core data structures
│   ├── sketchddd-parser/     # DSL parser
│   ├── sketchddd-codegen/    # Code generation
│   ├── sketchddd-viz/        # Visualization
│   ├── sketchddd-wasm/       # WASM bindings
│   └── sketchddd-cli/        # CLI tool
├── web/                       # React web application
├── docs/                      # Documentation
│   └── adr/                  # Architecture Decision Records
├── templates/                 # Domain templates
└── examples/                  # Example projects
```

## Making Changes

### For Bug Fixes
1. Create an issue describing the bug (if one doesn't exist)
2. Reference the issue in your PR
3. Include tests that reproduce the bug
4. Ensure all tests pass

### For New Features
1. Open an issue to discuss the feature first
2. Wait for maintainer feedback before starting work
3. Include documentation and tests
4. Update ADRs if making architectural decisions

### For Documentation
- Documentation improvements are always welcome
- Use clear, concise language
- Include examples where helpful

## Pull Request Process

1. Ensure your code follows the style guidelines
2. Update documentation as needed
3. Add tests for new functionality
4. Ensure all tests pass locally
5. Create a PR with a clear description
6. Reference any related issues
7. Wait for review and address feedback

### PR Title Format
- `feat: add new feature`
- `fix: resolve bug in parser`
- `docs: update contributing guide`
- `refactor: improve code structure`
- `test: add missing tests`
- `chore: update dependencies`

## Architecture Decision Records

We use ADRs to document significant architectural decisions. ADRs are stored in `docs/adr/`.

### When to Write an ADR
- Choosing a technology or framework
- Defining data structures or APIs
- Making trade-offs between approaches
- Any decision that future contributors should understand

### ADR Process
1. Copy `docs/adr/0000-adr-template.md`
2. Number it sequentially (e.g., `0005-your-decision.md`)
3. Fill in all sections
4. Submit as part of your PR
5. ADRs are reviewed alongside code

## Style Guidelines

### Rust
- Follow `rustfmt` formatting
- Use `clippy` for linting
- Document public APIs with doc comments
- Write tests for all public functions

### TypeScript/React
- Follow ESLint configuration
- Use TypeScript strict mode
- Prefer functional components with hooks
- Document complex logic with comments

### Commits
- Use conventional commit messages
- Keep commits focused and atomic
- Write clear commit descriptions

### Documentation
- Use Markdown for all documentation
- Include code examples
- Keep language clear and accessible

## Questions?

If you have questions, feel free to:
- Open a GitHub issue with the `question` label
- Start a discussion in GitHub Discussions

Thank you for contributing to SketchDDD!
