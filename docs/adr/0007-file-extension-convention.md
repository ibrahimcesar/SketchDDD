# ADR-0007: File Extension Convention (.sddd)

## Status
Accepted

## Context

SketchDDD models are stored as text files using the DSL defined in ADR-0006. We needed to choose a file extension that:

1. Is immediately recognizable as a SketchDDD file
2. Does not conflict with existing tools or extensions
3. Is short enough for practical use
4. Can be associated with syntax highlighting and IDE support
5. Works across all operating systems

Several options were considered:
- `.sketch` - Descriptive but conflicts with Sketch design tool
- `.sketchddd` - Full name but verbose (10 characters)
- `.sddd` - Short abbreviation (5 characters)
- `.ddd` - Too generic, may conflict with other DDD tools
- `.domain` - Descriptive but not specific to SketchDDD

## Decision
We adopt **`.sddd`** as the official file extension for SketchDDD DSL files.

### Rationale
1. **Short and memorable**: 5 characters including the dot
2. **Unique**: No known conflicts with existing tools
3. **Meaningful**: "s" for Sketch, "ddd" for Domain-Driven Design
4. **Practical**: Easy to type in CLI commands
5. **Sortable**: Files sort together in directory listings

### Usage Examples
```bash
# File naming convention
commerce.sddd
shipping.sddd
orders-context.sddd

# CLI usage
sketchddd check commerce.sddd
sketchddd codegen commerce.sddd --target rust
sketchddd viz *.sddd --output diagrams/

# In project structure
my-project/
├── domains/
│   ├── commerce.sddd
│   ├── shipping.sddd
│   └── billing.sddd
├── generated/
└── README.md
```

### MIME Type
The recommended MIME type is: `text/x-sddd`

For web servers and HTTP responses:

```
Content-Type: text/x-sddd; charset=utf-8
```

### File Associations

#### VS Code
In `settings.json`:
```json
{
  "files.associations": {
    "*.sddd": "sketchddd"
  }
}
```

#### JetBrains IDEs
File type pattern: `*.sddd`

#### GitHub/GitLab Linguist
Future: Submit `.sddd` to GitHub Linguist for syntax highlighting in repositories.

## Consequences

### Positive
- Clear identity for SketchDDD files
- No conflicts with existing tools
- Easy to search: `find . -name "*.sddd"`
- Enables IDE/editor support development
- Professional appearance in codebases

### Negative
- Not immediately obvious what "sddd" stands for
- Users must learn the convention
- Need to register with GitHub Linguist for syntax highlighting

### Neutral
- Extension is case-insensitive on Windows, case-sensitive on Unix

## Implementation Notes

### CLI Auto-Detection
The CLI should:
1. Accept explicit `.sddd` files: `sketchddd check foo.sddd`
2. Auto-detect in directories: `sketchddd check domains/`
3. Support globs: `sketchddd check **/*.sddd`
4. Warn on non-standard extensions

### Init Command
The `sketchddd init` command creates files with `.sddd` extension:
```bash
sketchddd init MyProject
# Creates: MyProject/myproject.sddd
```

## References
- [Issue #32: Document recommended .sddd file extension](https://github.com/ibrahimcesar/SketchDDD/issues/32)
- [Issue #33: Configure IDE/editor support for syntax highlighting](https://github.com/ibrahimcesar/SketchDDD/issues/33)
- [Issue #34: Add .sddd file extension auto-detection to CLI](https://github.com/ibrahimcesar/SketchDDD/issues/34)
- [ADR-0006: DSL Syntax Design](0006-dsl-syntax-design.md)
- [GitHub Linguist](https://github.com/github/linguist) - For future language registration
