<div align="center">

# SketchDDD

**Build Domain Models Visually or with Code**

[![Crates.io](https://img.shields.io/crates/v/sketchddd.svg)](https://crates.io/crates/sketchddd)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![CI](https://github.com/ibrahimcesar/SketchDDD/workflows/CI/badge.svg)](https://github.com/ibrahimcesar/SketchDDD/actions)

[Website](https://sketchddd.dev) • [Documentation](https://docs.sketchddd.dev) • [Visual Builder](https://app.sketchddd.dev)

</div>

---

## What is SketchDDD?

SketchDDD bridges the gap between **domain experts** and **developers** by providing two ways to build the same precise domain model:

| 👩‍💼 Domain Experts | 👨‍💻 Developers |
|-------------------|---------------|
| Visual drag-and-drop builder | Text-based DSL |
| Guided wizards | Full expressiveness |
| Templates to start fast | Version control friendly |
| Plain English rules | CI/CD integration |

Both interfaces produce the same model, validated by **category theory** to ensure precision.

---

## Quick Start

### Visual Builder

Visit [app.sketchddd.dev](https://app.sketchddd.dev) and start building.

### CLI
```bash
# Install
cargo install sketchddd

# Create a new project
sketchddd init my-domain

# Validate your model
sketchddd check my-domain.sketch

# Generate code
sketchddd codegen my-domain.sketch --target rust

# Start visual builder locally
sketchddd serve
```

---

## Example
```sketchddd
context Commerce {
  
  objects { Customer, Order, LineItem, Product, Money }
  
  morphisms {
    placedBy: Order -> Customer
    items: Order -> List<LineItem>
    product: LineItem -> Product
    price: LineItem -> Money
  }
  
  aggregate Order {
    root: Order
    contains: [LineItem]
    invariant: totalPrice = sum(items.price)
  }
  
  value Money {
    amount: Decimal
    currency: Currency
  }
}
```

---

## Features

- 🎨 **Visual Builder** - Drag-and-drop for non-technical users
- 📝 **Text DSL** - Full control for developers
- ✅ **Validation** - Catch errors before runtime
- 🔗 **Context Maps** - Model system integration
- 🏭 **Code Generation** - Rust, TypeScript, Kotlin, Python, Java, Clojure
- 📊 **Diagrams** - Auto-generated visualizations
- 📚 **Templates** - Start with common patterns

---

## Editor Support

First-class editor support for `.sddd` files:

| Editor | Status | Features |
|--------|--------|----------|
| **VS Code** | ✅ Available | Syntax highlighting, snippets, bracket matching |
| **Sublime Text** | ✅ Available | Syntax highlighting |
| **Vim/Neovim** | ✅ Available | Syntax highlighting, indentation |
| **Any LSP Client** | ✅ Available | Full LSP support |

### Language Server Protocol (LSP)

The `sketchddd-lsp` binary provides:
- Real-time diagnostics and error reporting
- Auto-completion for keywords, types, and morphisms
- Go to definition and find references
- Hover documentation
- Document symbols and outline
- Code formatting

```bash
# Install LSP server
cargo install sketchddd-lsp

# For Neovim (add to your config)
# lua: require('lspconfig').sketchddd.setup{}
```

See [editors/](editors/) for installation instructions

---

## Documentation

- [Getting Started](https://docs.sketchddd.dev/getting-started)
- [Visual Builder Guide](https://docs.sketchddd.dev/visual-builder)
- [DSL Reference](https://docs.sketchddd.dev/dsl-reference)
- [Theoretical Foundation](https://docs.sketchddd.dev/theory)

---

## Why Category Theory?

DDD concepts like "aggregate" and "bounded context" are often vague. SketchDDD uses **category theory** to give them precise mathematical definitions:

| DDD Concept | Mathematical Definition |
|-------------|------------------------|
| Bounded Context | Sketch (graph + equations + limits) |
| Aggregate | Limit cone with root |
| Value Object | Limit with structural equality |
| Context Map | Sketch morphism |

This precision enables **automated validation** and **code generation** that actually works.

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Open Discussions

We're building this in the open! Join the conversation:

- [Visual Builder Frontend Framework](https://github.com/ibrahimcesar/SketchDDD/discussions/31) - Help us choose React, Vue, Svelte, or Solid

---

## License

Licensed under either of:

- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

at your option.