# ADR-0003: Dual Interface - Visual Builder and Text DSL

## Status
Accepted

## Context
SketchDDD serves two distinct user groups:

1. **Domain Experts**: Business analysts, product managers, and domain specialists who understand the business but may not code. They need an intuitive, visual way to express domain knowledge.

2. **Developers**: Software engineers who prefer text-based tooling that integrates with their workflow (version control, CI/CD, code review).

Most existing DDD tools serve only one group, creating a translation gap where domain experts create diagrams that developers must manually convert to code.

## Decision
We will provide **two interfaces** to the same underlying model:

### 1. Visual Builder (Web Application)
- Drag-and-drop interface for building domain models
- Guided wizards for common tasks (creating entities, aggregates, etc.)
- Real-time validation with friendly error messages
- Template library for common patterns
- Export/import to DSL format

### 2. Text DSL + CLI
- Human-readable domain-specific language (`.sketch` files)
- Full expressiveness for complex models
- Version control friendly (diff, merge, review)
- CLI tool for validation, code generation, visualization
- CI/CD integration

Both interfaces produce the same semantic model:
```
Visual Builder ←→ Core Model ←→ Text DSL
```

## Consequences

### Positive
- **Accessibility**: Domain experts can participate directly in modeling
- **Developer experience**: Developers get text-based tooling they prefer
- **Collaboration**: Both groups work on the same model
- **No translation gap**: The model IS the source of truth
- **Flexibility**: Use either interface as appropriate for the task

### Negative
- **Development cost**: Two UIs to build and maintain
- **Sync complexity**: Must keep visual and text representations synchronized
- **Feature parity**: New features need implementation in both interfaces

### Neutral
- Users may develop preferences and only use one interface
- The sync requirement forces us to keep the model well-structured

## References
- Similar dual-interface tools: Figma (visual + code export), Prisma (schema + studio)
- Research on cognitive load in visual vs. text programming
