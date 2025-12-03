# Changelog

All notable changes to SketchDDD will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-12-02

### Added

#### Core Language
- Complete SketchDDD DSL implementation with category theory foundations
- Support for bounded contexts, entities, value objects, enums, and aggregates
- Morphisms with cardinality annotations (`@one`, `@many`, `@optional`)
- Context maps with integration patterns (ACL, Open Host, Conformist, etc.)
- Path equations for expressing domain invariants
- Sum types (tagged unions) with payload support

#### Parser
- Full parser for `.sddd` files using pest grammar
- Rich error messages with source locations using ariadne
- "Did you mean?" suggestions for typos
- Pretty-printing for AST round-trips

#### Code Generation
- **Rust**: Structs, enums, derives, documentation
- **TypeScript**: Interfaces, types, Zod schemas, branded types
- **Kotlin**: Data classes, sealed classes, kotlinx.serialization
- **Python**: Dataclasses, Pydantic models, enums
- **Java**: Records (Java 17+), sealed interfaces
- **Haskell**: Data types, Aeson instances
- **Clojure**: Records, specs

#### Visualization
- Graphviz DOT output for domain diagrams
- Mermaid diagram generation

#### CLI (`sketchddd`)
- `check` - Validate .sddd files with detailed diagnostics
- `codegen` - Generate code for multiple target languages
- `viz` - Generate diagrams (Graphviz, Mermaid)
- `init` - Create new projects from templates
- `template` - Manage project templates
- `export/import` - JSON serialization
- `serve` - Local development server (placeholder)

#### WASM Support
- Full parser and validator available in browsers
- Code generation in browser
- Integration with web playground

#### Editor Support
- VS Code extension with syntax highlighting and snippets
- Sublime Text syntax definition
- Vim/Neovim syntax, indentation, and filetype detection
- TextMate grammar for broad editor compatibility

#### Language Server Protocol (LSP)
- Real-time diagnostics
- Auto-completion for keywords, types, and morphisms
- Go to definition
- Find references
- Document symbols
- Hover information
- Code formatting

#### Documentation
- Comprehensive user guide with mdBook
- Language specification
- API reference
- Error code documentation
- Templates and examples

#### CI/CD
- GitHub Actions for CI (multi-platform, multi-Rust-version)
- Security auditing with rustsec and cargo-deny
- Code coverage with Codecov
- Automated dependency updates with Dependabot
- Release workflow for binaries and crates.io publishing

### Security
- cargo-deny configuration for license and advisory checks
- Weekly security audit schedule

## [0.1.0] - 2024-11-01

### Added
- Initial project structure
- Basic parser prototype
- Core domain model types
