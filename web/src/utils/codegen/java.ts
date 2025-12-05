import type { ContextData, EntityNode, ValueNode, EnumNode, AggregateNode } from '@/types';
import type { CodeGenOptions, GeneratedFile } from './types';

function mapTypeToJava(type: string): string {
  const typeMap: Record<string, string> = {
    string: 'String',
    String: 'String',
    int: 'int',
    integer: 'int',
    Int: 'Integer',
    Integer: 'Integer',
    float: 'float',
    Float: 'Float',
    double: 'double',
    Double: 'Double',
    number: 'double',
    Number: 'Double',
    boolean: 'boolean',
    Boolean: 'Boolean',
    bool: 'boolean',
    Bool: 'Boolean',
    date: 'LocalDate',
    Date: 'LocalDate',
    datetime: 'LocalDateTime',
    DateTime: 'LocalDateTime',
    uuid: 'UUID',
    UUID: 'UUID',
  };
  return typeMap[type] || type;
}

function capitalize(str: string): string {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

function generateEntity(node: EntityNode, options: CodeGenOptions): string {
  const lines: string[] = [];
  const packageName = options.packageName || 'com.example.domain';

  lines.push(`package ${packageName};`);
  lines.push('');
  lines.push('import java.util.UUID;');
  lines.push('import java.util.Objects;');
  lines.push('');

  if (options.includeComments) {
    lines.push(`/**`);
    lines.push(` * Entity: ${node.name}`);
    lines.push(` * An entity with a unique identity`);
    lines.push(` */`);
  }

  lines.push(`public class ${node.name} {`);
  lines.push('');

  // Fields
  lines.push(`    private final UUID id;`);
  node.fields.forEach(field => {
    const javaType = mapTypeToJava(field.type);
    lines.push(`    private ${javaType} ${field.name};`);
  });

  lines.push('');

  // Constructor
  lines.push(`    public ${node.name}() {`);
  lines.push(`        this.id = UUID.randomUUID();`);
  lines.push(`    }`);

  lines.push('');

  lines.push(`    public ${node.name}(UUID id) {`);
  lines.push(`        this.id = id;`);
  lines.push(`    }`);

  lines.push('');

  // Getters and setters
  lines.push(`    public UUID getId() {`);
  lines.push(`        return id;`);
  lines.push(`    }`);

  node.fields.forEach(field => {
    const javaType = mapTypeToJava(field.type);
    const capName = capitalize(field.name);

    lines.push('');
    lines.push(`    public ${javaType} get${capName}() {`);
    lines.push(`        return ${field.name};`);
    lines.push(`    }`);

    lines.push('');
    lines.push(`    public void set${capName}(${javaType} ${field.name}) {`);
    lines.push(`        this.${field.name} = ${field.name};`);
    lines.push(`    }`);
  });

  lines.push('');

  // equals and hashCode based on id
  lines.push(`    @Override`);
  lines.push(`    public boolean equals(Object o) {`);
  lines.push(`        if (this == o) return true;`);
  lines.push(`        if (o == null || getClass() != o.getClass()) return false;`);
  lines.push(`        ${node.name} that = (${node.name}) o;`);
  lines.push(`        return Objects.equals(id, that.id);`);
  lines.push(`    }`);

  lines.push('');
  lines.push(`    @Override`);
  lines.push(`    public int hashCode() {`);
  lines.push(`        return Objects.hash(id);`);
  lines.push(`    }`);

  lines.push(`}`);

  return lines.join('\n');
}

function generateValueObject(node: ValueNode, options: CodeGenOptions): string {
  const lines: string[] = [];
  const packageName = options.packageName || 'com.example.domain';

  lines.push(`package ${packageName};`);
  lines.push('');
  lines.push('import java.util.Objects;');
  lines.push('');

  if (options.includeComments) {
    lines.push(`/**`);
    lines.push(` * Value Object: ${node.name}`);
    lines.push(` * An immutable value type identified by its attributes`);
    lines.push(` */`);
  }

  lines.push(`public final class ${node.name} {`);
  lines.push('');

  // Final fields
  node.fields.forEach(field => {
    const javaType = mapTypeToJava(field.type);
    lines.push(`    private final ${javaType} ${field.name};`);
  });

  lines.push('');

  // Constructor
  const params = node.fields.map(f => `${mapTypeToJava(f.type)} ${f.name}`).join(', ');
  lines.push(`    public ${node.name}(${params}) {`);
  node.fields.forEach(field => {
    lines.push(`        this.${field.name} = ${field.name};`);
  });
  lines.push(`    }`);

  // Getters only (no setters for immutability)
  node.fields.forEach(field => {
    const javaType = mapTypeToJava(field.type);
    const capName = capitalize(field.name);

    lines.push('');
    lines.push(`    public ${javaType} get${capName}() {`);
    lines.push(`        return ${field.name};`);
    lines.push(`    }`);
  });

  lines.push('');

  // equals based on all fields
  lines.push(`    @Override`);
  lines.push(`    public boolean equals(Object o) {`);
  lines.push(`        if (this == o) return true;`);
  lines.push(`        if (o == null || getClass() != o.getClass()) return false;`);
  lines.push(`        ${node.name} that = (${node.name}) o;`);
  const equalsCondition = node.fields.map(f => `Objects.equals(${f.name}, that.${f.name})`).join(' && ') || 'true';
  lines.push(`        return ${equalsCondition};`);
  lines.push(`    }`);

  lines.push('');
  lines.push(`    @Override`);
  lines.push(`    public int hashCode() {`);
  const hashFields = node.fields.map(f => f.name).join(', ') || '';
  lines.push(`        return Objects.hash(${hashFields});`);
  lines.push(`    }`);

  lines.push(`}`);

  return lines.join('\n');
}

function generateEnum(node: EnumNode, options: CodeGenOptions): string {
  const lines: string[] = [];
  const packageName = options.packageName || 'com.example.domain';

  lines.push(`package ${packageName};`);
  lines.push('');

  if (options.includeComments) {
    lines.push(`/**`);
    lines.push(` * Enum: ${node.name}`);
    lines.push(` */`);
  }

  lines.push(`public enum ${node.name} {`);
  node.variants.forEach((variant, index) => {
    const comma = index < node.variants.length - 1 ? ',' : ';';
    lines.push(`    ${variant.name.toUpperCase()}${comma}`);
  });
  lines.push(`}`);

  return lines.join('\n');
}

function generateAggregate(node: AggregateNode, nodes: Record<string, { node: { name: string } }>, options: CodeGenOptions): string {
  const lines: string[] = [];
  const packageName = options.packageName || 'com.example.domain';

  lines.push(`package ${packageName};`);
  lines.push('');
  lines.push('import java.util.List;');
  lines.push('import java.util.ArrayList;');
  lines.push('');

  if (options.includeComments) {
    lines.push(`/**`);
    lines.push(` * Aggregate: ${node.name}`);
    const rootNode = nodes[node.rootId]?.node;
    if (rootNode) {
      lines.push(` * Root Entity: ${rootNode.name}`);
    }
    if (node.invariants.length > 0) {
      lines.push(` * `);
      lines.push(` * Invariants:`);
      node.invariants.forEach(inv => {
        lines.push(` * - ${inv}`);
      });
    }
    lines.push(` */`);
  }

  const rootNode = nodes[node.rootId]?.node;

  lines.push(`public class ${node.name}Aggregate {`);
  lines.push('');

  if (rootNode) {
    lines.push(`    private final ${rootNode.name} root;`);
  }

  node.memberIds.forEach(id => {
    const memberNode = nodes[id]?.node;
    if (memberNode) {
      lines.push(`    private final List<${memberNode.name}> ${memberNode.name.toLowerCase()}s;`);
    }
  });

  lines.push('');

  // Constructor
  lines.push(`    public ${node.name}Aggregate(${rootNode ? `${rootNode.name} root` : ''}) {`);
  if (rootNode) {
    lines.push(`        this.root = root;`);
  }
  node.memberIds.forEach(id => {
    const memberNode = nodes[id]?.node;
    if (memberNode) {
      lines.push(`        this.${memberNode.name.toLowerCase()}s = new ArrayList<>();`);
    }
  });
  lines.push(`    }`);

  lines.push('');

  // Getters
  if (rootNode) {
    lines.push(`    public ${rootNode.name} getRoot() {`);
    lines.push(`        return root;`);
    lines.push(`    }`);
  }

  node.memberIds.forEach(id => {
    const memberNode = nodes[id]?.node;
    if (memberNode) {
      lines.push('');
      lines.push(`    public List<${memberNode.name}> get${capitalize(memberNode.name)}s() {`);
      lines.push(`        return new ArrayList<>(${memberNode.name.toLowerCase()}s);`);
      lines.push(`    }`);

      lines.push('');
      lines.push(`    public void add${memberNode.name}(${memberNode.name} item) {`);
      lines.push(`        this.${memberNode.name.toLowerCase()}s.add(item);`);
      lines.push(`    }`);
    }
  });

  lines.push(`}`);

  return lines.join('\n');
}

export function generateJava(context: ContextData, options: CodeGenOptions): GeneratedFile[] {
  const files: GeneratedFile[] = [];
  const nodeEntries = Object.entries(context.nodes);

  // Generate each type as a separate file
  nodeEntries.forEach(([, data]) => {
    let content = '';
    let filename = '';

    switch (data.node.kind) {
      case 'entity':
        content = generateEntity(data.node as EntityNode, options);
        filename = `${data.node.name}.java`;
        break;
      case 'value':
        content = generateValueObject(data.node as ValueNode, options);
        filename = `${data.node.name}.java`;
        break;
      case 'enum':
        content = generateEnum(data.node as EnumNode, options);
        filename = `${data.node.name}.java`;
        break;
      case 'aggregate':
        content = generateAggregate(data.node as AggregateNode, context.nodes, options);
        filename = `${data.node.name}Aggregate.java`;
        break;
    }

    if (content && filename) {
      files.push({
        filename,
        content,
        language: 'java',
      });
    }
  });

  return files;
}
