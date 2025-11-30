# ADR-0012: Template Format Specification

## Status
Accepted

## Context
SketchDDD needs a template system that allows users to quickly start modeling common domain patterns. Templates serve multiple purposes:

1. **Quick Start** - Reduce time to first working model
2. **Learning** - Show best practices and patterns
3. **Consistency** - Establish organizational standards
4. **Reusability** - Share domain patterns across projects

Templates must work across both the CLI and Visual Builder, supporting:
- Predefined templates shipped with SketchDDD
- User-created custom templates
- Organization-shared templates
- Template parameters for customization

## Decision

### Template Package Structure

Templates are packaged as directories with a manifest:

```
templates/
├── ecommerce/
│   ├── template.toml          # Template manifest
│   ├── model.sddd             # Main model file
│   ├── thumbnail.png          # Preview image (optional)
│   └── README.md              # Documentation
├── healthcare/
│   ├── template.toml
│   ├── model.sddd
│   └── partials/
│       ├── patient.sddd
│       └── provider.sddd
└── inventory/
    ├── template.toml
    └── model.sddd
```

### Template Manifest (template.toml)

```toml
[template]
name = "E-Commerce"
version = "1.0.0"
description = "Complete e-commerce domain with orders, products, and customers"
author = "SketchDDD Team"
license = "MIT"
category = "retail"
tags = ["commerce", "orders", "products", "customers"]

# Minimum SketchDDD version required
requires = ">=0.1.0"

[metadata]
# For template browser display
icon = "shopping-cart"
color = "#4CAF50"
difficulty = "beginner"
estimated_entities = 12
estimated_contexts = 3

[parameters]
# Template parameters for customization
[parameters.company_name]
type = "string"
description = "Your company name"
default = "Acme"
required = true

[parameters.multi_currency]
type = "boolean"
description = "Enable multi-currency support"
default = false

[parameters.inventory_tracking]
type = "enum"
description = "Inventory tracking method"
options = ["none", "simple", "warehouse"]
default = "simple"

[files]
# Main model file
main = "model.sddd"

# Additional files to include
include = [
  "partials/*.sddd"
]

# Files to generate from templates
[files.generated]
"README.md" = "templates/readme.md.hbs"

[dependencies]
# Other templates this one extends
extends = []
imports = []
```

### Template Content with Parameters

Templates use Handlebars syntax for parameterization:

```sketchddd
// model.sddd
// {{template.name}} Domain Model
// Generated for: {{parameters.company_name}}

context {{parameters.company_name}}Commerce {
  objects {
    Customer,
    Order,
    Product,
    {{#if parameters.inventory_tracking}}
    Inventory,
    {{/if}}
    {{#if parameters.multi_currency}}
    Currency,
    ExchangeRate,
    {{/if}}
  }

  entity Customer {
    id: CustomerId
    name: String
    email: Email
    {{#if parameters.multi_currency}}
    preferred_currency: Currency
    {{/if}}
  }

  value Money {
    amount: Decimal
    {{#if parameters.multi_currency}}
    currency: Currency
    {{else}}
    // Single currency: USD
    {{/if}}
  }

  {{#switch parameters.inventory_tracking}}
  {{#case "simple"}}
  entity Product {
    id: ProductId
    name: String
    price: Money
    stock_quantity: Integer
  }
  {{/case}}
  {{#case "warehouse"}}
  entity Product {
    id: ProductId
    name: String
    price: Money
  }

  entity Inventory {
    product: ProductId
    warehouse: WarehouseId
    quantity: Integer
    reserved: Integer
  }
  {{/case}}
  {{#default}}
  entity Product {
    id: ProductId
    name: String
    price: Money
  }
  {{/default}}
  {{/switch}}

  aggregate Order {
    root: Order
    contains: [LineItem]
    invariant: total = sum(items.subtotal)
  }
}
```

### Predefined Templates

SketchDDD ships with these built-in templates:

| Template | Category | Description | Contexts |
|----------|----------|-------------|----------|
| **Minimal** | starter | Empty starting point | 1 |
| **E-Commerce** | retail | Orders, products, customers | 3 |
| **Healthcare** | medical | Patients, providers, appointments | 4 |
| **Banking** | finance | Accounts, transactions, customers | 3 |
| **Inventory** | logistics | Products, warehouses, stock | 2 |
| **Project Management** | business | Projects, tasks, teams | 2 |
| **Event Sourcing** | architecture | CQRS/ES patterns | 2 |
| **Multi-Tenant SaaS** | architecture | Tenant isolation patterns | 3 |

### Template Discovery

Templates are discovered from multiple sources:

```
Priority (highest to lowest):
1. Project local:     ./templates/
2. User home:         ~/.sketchddd/templates/
3. Organization:      $SKETCHDDD_TEMPLATES_PATH
4. Built-in:          (bundled with installation)
```

### CLI Integration

```bash
# List available templates (local + built-in)
sketchddd template list

# List community templates from GitHub
sketchddd template list --remote

# Show template details
sketchddd template info ecommerce

# Create project from template
sketchddd init --template ecommerce

# Create with parameters
sketchddd init --template ecommerce \
  --set company_name=MyCorp \
  --set multi_currency=true

# Interactive mode
sketchddd init --template ecommerce --interactive

# Install a community template from GitHub
sketchddd template install gaming
sketchddd template install real-estate

# Install specific version/branch
sketchddd template install gaming --ref v1.0.0

# Update all installed community templates
sketchddd template update

# Remove an installed template
sketchddd template remove gaming

# Create custom template from existing project
sketchddd template create --name my-template --from ./my-project
```

### Template Installation

Community templates are fetched directly from GitHub:

```
GET https://api.github.com/repos/ibrahimcesar/SketchDDD/contents/templates/community/{name}
```

**Installation flow:**
1. CLI queries GitHub API for template directory contents
2. Downloads `template.toml` and validates compatibility
3. Downloads all template files to `~/.sketchddd/templates/{name}/`
4. Template is now available for `sketchddd init --template {name}`

**Offline support:**
- Built-in templates are always available (bundled with CLI)
- Once installed, community templates work offline
- `--remote` flag requires internet connection

### Visual Builder Integration

The Visual Builder provides a template browser:

```typescript
// Template browser component structure
interface TemplateBrowserProps {
  onSelect: (template: Template) => void;
  filter?: {
    category?: string;
    tags?: string[];
    difficulty?: 'beginner' | 'intermediate' | 'advanced';
  };
}

interface Template {
  id: string;
  name: string;
  description: string;
  category: string;
  tags: string[];
  thumbnail?: string;
  parameters: TemplateParameter[];
  metadata: TemplateMetadata;
}

// Template selection flow
┌─────────────────────────────────────────────────────────────┐
│                    New Project                              │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐         │
│  │ Minimal │  │Commerce │  │Healthcare│  │ Banking │        │
│  │   📄    │  │   🛒    │  │    🏥    │  │   🏦    │         │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘         │
│                                                             │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐       │
│  │Inventory│  │ Project │  │  Event  │  │  SaaS   │       │
│  │   📦    │  │  📋      │  │Sourcing │  │   ☁️    │       │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘       │
│                                                             │
│  [Search templates...]                    [Filter ▼]       │
└─────────────────────────────────────────────────────────────┘
```

### Parameter UI Generation

Parameters in template.toml automatically generate UI:

```typescript
// Auto-generated form from template parameters
function TemplateParameterForm({ template, onSubmit }) {
  return (
    <form>
      {template.parameters.map(param => (
        <ParameterInput key={param.name} param={param} />
      ))}
      <button type="submit">Create Project</button>
    </form>
  );
}

// Parameter types map to form controls:
// string  → text input
// boolean → checkbox
// enum    → select dropdown
// number  → number input
// array   → multi-select
```

### Template Validation

Templates are validated before use:

```rust
pub struct TemplateValidator {
    errors: Vec<TemplateError>,
}

impl TemplateValidator {
    pub fn validate(&mut self, template: &Template) -> ValidationResult {
        // Check manifest completeness
        self.validate_manifest(&template.manifest);

        // Validate main file exists and is valid .sddd
        self.validate_main_file(&template.main_file);

        // Check all parameters are used
        self.validate_parameter_usage(&template);

        // Verify included files exist
        self.validate_includes(&template.includes);

        // Check template renders with default values
        self.validate_default_rendering(&template);

        ValidationResult {
            valid: self.errors.is_empty(),
            errors: self.errors.clone(),
        }
    }
}
```

### Community Templates via GitHub

Community templates are contributed via pull requests to the main repository:

```
github.com/ibrahimcesar/SketchDDD/
└── templates/
    ├── builtin/           # Official templates (maintained by core team)
    │   ├── minimal/
    │   ├── ecommerce/
    │   └── healthcare/
    └── community/         # Community-contributed templates
        ├── gaming/
        ├── real-estate/
        └── logistics/
```

**Contribution Process:**
1. Fork the repository
2. Add template to `templates/community/`
3. Ensure template passes validation (`sketchddd template validate`)
4. Submit PR with template documentation
5. Core team reviews and merges

**Benefits of GitHub-based approach:**
- No infrastructure to maintain
- Standard PR review process
- Version control and history
- Easy to fork and customize
- Community can contribute without additional accounts

## Consequences

### Positive
- Quick start for new projects with best-practice patterns
- Consistent domain modeling across organizations
- Educational value through well-documented templates
- Extensible system for custom and community templates
- Parameter system enables template customization
- Works seamlessly in CLI and Visual Builder

### Negative
- Template maintenance burden as DSL evolves
- Handlebars adds complexity to template authoring
- Relies on GitHub for community contributions
- Version compatibility between templates and SketchDDD

### Neutral
- Templates are optional - users can start from scratch
- Built-in templates cover common use cases
- Custom templates require TOML + Handlebars knowledge


## References

### Issues
- [Issue #19: Template library with common patterns](https://github.com/ibrahimcesar/SketchDDD/issues/19)
- [Issue #39: Implement template list command](https://github.com/ibrahimcesar/SketchDDD/issues/39)
- [Issue #40: Implement template install/update/remove commands](https://github.com/ibrahimcesar/SketchDDD/issues/40)
- [Issue #41: Implement template info and validate commands](https://github.com/ibrahimcesar/SketchDDD/issues/41)
- [Issue #42: Implement template create command](https://github.com/ibrahimcesar/SketchDDD/issues/42)

### Related ADRs
- [ADR-0007: File Extension Convention](0007-file-extension-convention.md)

### External Resources
- [Handlebars](https://handlebarsjs.com/) - Template language
- [Cargo Templates](https://doc.rust-lang.org/cargo/reference/templates.html) - Inspiration
- [Yeoman Generators](https://yeoman.io/generators/) - Template system reference
