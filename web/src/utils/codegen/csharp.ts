import type { ContextData, EntityNode, ValueNode, EnumNode, AggregateNode } from '@/types';
import type { CodeGenOptions, GeneratedFile } from './types';

function mapTypeToCSharp(type: string): string {
  const typeMap: Record<string, string> = {
    string: 'string',
    String: 'string',
    int: 'int',
    integer: 'int',
    Int: 'int',
    Integer: 'int',
    float: 'float',
    Float: 'float',
    double: 'double',
    Double: 'double',
    number: 'double',
    Number: 'double',
    boolean: 'bool',
    Boolean: 'bool',
    bool: 'bool',
    Bool: 'bool',
    date: 'DateOnly',
    Date: 'DateOnly',
    datetime: 'DateTime',
    DateTime: 'DateTime',
    uuid: 'Guid',
    UUID: 'Guid',
  };
  return typeMap[type] || type;
}

function generateEntity(node: EntityNode, options: CodeGenOptions): string {
  const lines: string[] = [];
  const namespace = options.namespace || 'Domain';

  lines.push(`namespace ${namespace};`);
  lines.push('');

  if (options.includeComments) {
    lines.push(`/// <summary>`);
    lines.push(`/// Entity: ${node.name}`);
    lines.push(`/// An entity with a unique identity`);
    lines.push(`/// </summary>`);
  }

  lines.push(`public class ${node.name}`);
  lines.push(`{`);

  // Id property
  lines.push(`    public Guid Id { get; init; } = Guid.NewGuid();`);
  lines.push('');

  // Properties
  node.fields.forEach(field => {
    const csType = mapTypeToCSharp(field.type);
    const nullable = field.optional ? '?' : '';
    const required = field.optional ? '' : ' required';
    lines.push(`    public${required} ${csType}${nullable} ${field.name.charAt(0).toUpperCase() + field.name.slice(1)} { get; set; }`);
  });

  lines.push('');

  // Equality based on Id
  lines.push(`    public override bool Equals(object? obj)`);
  lines.push(`    {`);
  lines.push(`        if (obj is not ${node.name} other) return false;`);
  lines.push(`        return Id == other.Id;`);
  lines.push(`    }`);

  lines.push('');
  lines.push(`    public override int GetHashCode() => Id.GetHashCode();`);

  lines.push(`}`);

  return lines.join('\n');
}

function generateValueObject(node: ValueNode, options: CodeGenOptions): string {
  const lines: string[] = [];
  const namespace = options.namespace || 'Domain';

  lines.push(`namespace ${namespace};`);
  lines.push('');

  if (options.includeComments) {
    lines.push(`/// <summary>`);
    lines.push(`/// Value Object: ${node.name}`);
    lines.push(`/// An immutable value type identified by its attributes`);
    lines.push(`/// </summary>`);
  }

  // Use record for value objects (immutable by default)
  const params = node.fields
    .map(f => {
      const csType = mapTypeToCSharp(f.type);
      const nullable = f.optional ? '?' : '';
      return `${csType}${nullable} ${f.name.charAt(0).toUpperCase() + f.name.slice(1)}`;
    })
    .join(', ');

  lines.push(`public record ${node.name}(${params});`);

  return lines.join('\n');
}

function generateEnum(node: EnumNode, options: CodeGenOptions): string {
  const lines: string[] = [];
  const namespace = options.namespace || 'Domain';

  lines.push(`namespace ${namespace};`);
  lines.push('');

  if (options.includeComments) {
    lines.push(`/// <summary>`);
    lines.push(`/// Enum: ${node.name}`);
    lines.push(`/// </summary>`);
  }

  lines.push(`public enum ${node.name}`);
  lines.push(`{`);
  node.variants.forEach((variant, index) => {
    const comma = index < node.variants.length - 1 ? ',' : '';
    lines.push(`    ${variant.name}${comma}`);
  });
  lines.push(`}`);

  return lines.join('\n');
}

function generateAggregate(node: AggregateNode, nodes: Record<string, { node: { name: string } }>, options: CodeGenOptions): string {
  const lines: string[] = [];
  const namespace = options.namespace || 'Domain';

  lines.push(`namespace ${namespace};`);
  lines.push('');

  if (options.includeComments) {
    lines.push(`/// <summary>`);
    lines.push(`/// Aggregate: ${node.name}`);
    const rootNode = nodes[node.rootId]?.node;
    if (rootNode) {
      lines.push(`/// Root Entity: ${rootNode.name}`);
    }
    if (node.invariants.length > 0) {
      lines.push(`/// `);
      lines.push(`/// Invariants:`);
      node.invariants.forEach(inv => {
        lines.push(`/// - ${inv}`);
      });
    }
    lines.push(`/// </summary>`);
  }

  const rootNode = nodes[node.rootId]?.node;

  lines.push(`public class ${node.name}Aggregate`);
  lines.push(`{`);

  // Private backing fields
  node.memberIds.forEach(id => {
    const memberNode = nodes[id]?.node;
    if (memberNode) {
      lines.push(`    private readonly List<${memberNode.name}> _${memberNode.name.toLowerCase()}s = new();`);
    }
  });

  if (node.memberIds.length > 0) {
    lines.push('');
  }

  // Properties
  if (rootNode) {
    lines.push(`    public required ${rootNode.name} Root { get; init; }`);
  }

  node.memberIds.forEach(id => {
    const memberNode = nodes[id]?.node;
    if (memberNode) {
      lines.push(`    public IReadOnlyList<${memberNode.name}> ${memberNode.name}s => _${memberNode.name.toLowerCase()}s.AsReadOnly();`);
    }
  });

  // Methods for aggregate operations
  node.memberIds.forEach(id => {
    const memberNode = nodes[id]?.node;
    if (memberNode) {
      lines.push('');
      lines.push(`    public void Add${memberNode.name}(${memberNode.name} item)`);
      lines.push(`    {`);
      lines.push(`        // TODO: Validate invariants before adding`);
      lines.push(`        _${memberNode.name.toLowerCase()}s.Add(item);`);
      lines.push(`    }`);

      lines.push('');
      lines.push(`    public bool Remove${memberNode.name}(${memberNode.name} item)`);
      lines.push(`    {`);
      lines.push(`        return _${memberNode.name.toLowerCase()}s.Remove(item);`);
      lines.push(`    }`);
    }
  });

  lines.push(`}`);

  return lines.join('\n');
}

export function generateCSharp(context: ContextData, options: CodeGenOptions): GeneratedFile[] {
  const files: GeneratedFile[] = [];
  const nodeEntries = Object.entries(context.nodes);

  // Generate each type as a separate file
  nodeEntries.forEach(([, data]) => {
    let content = '';
    let filename = '';

    switch (data.node.kind) {
      case 'entity':
        content = generateEntity(data.node as EntityNode, options);
        filename = `${data.node.name}.cs`;
        break;
      case 'value':
        content = generateValueObject(data.node as ValueNode, options);
        filename = `${data.node.name}.cs`;
        break;
      case 'enum':
        content = generateEnum(data.node as EnumNode, options);
        filename = `${data.node.name}.cs`;
        break;
      case 'aggregate':
        content = generateAggregate(data.node as AggregateNode, context.nodes, options);
        filename = `${data.node.name}Aggregate.cs`;
        break;
    }

    if (content && filename) {
      files.push({
        filename,
        content,
        language: 'csharp',
      });
    }
  });

  return files;
}
