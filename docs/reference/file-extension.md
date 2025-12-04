# File Extension: .sddd

SketchDDD uses the **`.sddd`** file extension for all domain model files.

## Why .sddd?

| Aspect | Benefit |
|--------|---------|
| **Short** | Only 5 characters including the dot |
| **Unique** | No conflicts with existing tools |
| **Meaningful** | "s" for Sketch + "ddd" for Domain-Driven Design |
| **Practical** | Easy to type in CLI commands |

## File Naming Conventions

### Recommended Patterns

```
commerce.sddd           # Context name
orders-context.sddd     # Descriptive with suffix
shipping.sddd           # Service/domain name
user-management.sddd    # Feature area
```

### Project Structure

```
my-project/
├── domains/
│   ├── commerce.sddd      # E-commerce context
│   ├── shipping.sddd      # Shipping context
│   └── billing.sddd       # Billing context
├── generated/
│   ├── rust/
│   └── typescript/
└── README.md
```

## CLI Usage

```bash
# Check a specific file
sketchddd check commerce.sddd

# Check all .sddd files in a directory
sketchddd check domains/

# Generate code from .sddd file
sketchddd codegen commerce.sddd --target rust

# Visualize all domain models
sketchddd viz *.sddd --output diagrams/
```

## Auto-Detection

The CLI automatically detects `.sddd` files:

```bash
# In a directory with domain.sddd
cd my-project
sketchddd check    # Auto-detects .sddd files
```

## Editor Configuration

### VS Code

The SketchDDD extension automatically associates `.sddd` files. For manual configuration:

```json
{
  "files.associations": {
    "*.sddd": "sketchddd"
  }
}
```

### Vim/Neovim

File type detection is automatic with the plugin. For manual setup:

```vim
autocmd BufRead,BufNewFile *.sddd set filetype=sketchddd
```

### Sublime Text

Install the SketchDDD syntax package, which handles `.sddd` files automatically.

## MIME Type

For web servers and HTTP responses:

```
Content-Type: text/x-sddd; charset=utf-8
```

### Example .htaccess

```apache
AddType text/x-sddd .sddd
```

### Example nginx

```nginx
types {
    text/x-sddd sddd;
}
```

## Git Configuration

Add syntax highlighting in Git diffs:

```gitattributes
*.sddd linguist-language=SketchDDD
*.sddd diff=sddd
```

## See Also

- [ADR-0007: File Extension Convention](../adr/0007-file-extension-convention.md)
- [Getting Started](../getting-started/quick-start.md)
- [Language Overview](../language/overview.md)
