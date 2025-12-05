import type { ContextData, EntityNode, ValueNode, EnumNode, AggregateNode, Morphism } from '@/types';
import type { CodeGenOptions, GeneratedFile } from './types';

function mapTypeToTS(type: string): string {
  const typeMap: Record<string, string> = {
    string: 'string',
    String: 'string',
    int: 'number',
    integer: 'number',
    Int: 'number',
    Integer: 'number',
    float: 'number',
    Float: 'number',
    double: 'number',
    Double: 'number',
    number: 'number',
    Number: 'number',
    boolean: 'boolean',
    Boolean: 'boolean',
    bool: 'boolean',
    Bool: 'boolean',
    date: 'Date',
    Date: 'Date',
    datetime: 'Date',
    DateTime: 'Date',
    uuid: 'string',
    UUID: 'string',
  };
  return typeMap[type] || type;
}

function generateEntity(node: EntityNode, morphisms: Morphism[], nodes: Record<string, { node: { name: string } }>, options: CodeGenOptions): string {
  const lines: string[] = [];

  if (options.includeComments) {
    lines.push(`/**`);
    lines.push(` * Entity: ${node.name}`);
    lines.push(` * An entity with a unique identity`);
    lines.push(` */`);
  }

  if (options.generateInterfaces) {
    lines.push(`export interface ${node.name} {`);

    // Add id field for entities
    lines.push(`  readonly id: string;`);

    // Add fields
    node.fields.forEach(field => {
      const tsType = mapTypeToTS(field.type);
      const optional = field.optional ? '?' : '';
      lines.push(`  ${field.name}${optional}: ${tsType};`);
    });

    // Add relationships from morphisms
    morphisms
      .filter(m => m.sourceId === node.name || Object.entries(nodes).find(([id]) => id === m.sourceId)?.[1]?.node.name === node.name)
      .forEach(m => {
        const targetNode = Object.entries(nodes).find(([id]) => id === m.targetId)?.[1]?.node;
        if (targetNode) {
          const cardinality = m.cardinality === 'many' ? '[]' : m.cardinality === 'optional' ? ' | null' : '';
          lines.push(`  ${m.name}${m.cardinality === 'optional' ? '?' : ''}: ${targetNode.name}${cardinality === '[]' ? '[]' : cardinality};`);
        }
      });

    lines.push(`}`);
  }

  if (options.generateClasses) {
    lines.push('');
    lines.push(`export class ${node.name}Impl implements ${node.name} {`);
    lines.push(`  readonly id: string;`);

    node.fields.forEach(field => {
      const tsType = mapTypeToTS(field.type);
      const optional = field.optional ? '?' : '';
      lines.push(`  ${field.name}${optional}: ${tsType};`);
    });

    lines.push('');
    lines.push(`  constructor(data: Omit<${node.name}, 'id'> & { id?: string }) {`);
    lines.push(`    this.id = data.id ?? crypto.randomUUID();`);
    node.fields.forEach(field => {
      lines.push(`    this.${field.name} = data.${field.name};`);
    });
    lines.push(`  }`);
    lines.push(`}`);
  }

  return lines.join('\n');
}

function generateValueObject(node: ValueNode, options: CodeGenOptions): string {
  const lines: string[] = [];

  if (options.includeComments) {
    lines.push(`/**`);
    lines.push(` * Value Object: ${node.name}`);
    lines.push(` * An immutable value type identified by its attributes`);
    lines.push(` */`);
  }

  if (options.generateInterfaces) {
    lines.push(`export interface ${node.name} {`);
    node.fields.forEach(field => {
      const tsType = mapTypeToTS(field.type);
      const optional = field.optional ? '?' : '';
      lines.push(`  readonly ${field.name}${optional}: ${tsType};`);
    });
    lines.push(`}`);
  }

  if (options.generateClasses) {
    lines.push('');
    lines.push(`export class ${node.name}Impl implements ${node.name} {`);
    node.fields.forEach(field => {
      const tsType = mapTypeToTS(field.type);
      lines.push(`  readonly ${field.name}: ${tsType};`);
    });

    lines.push('');
    lines.push(`  constructor(data: ${node.name}) {`);
    node.fields.forEach(field => {
      lines.push(`    this.${field.name} = data.${field.name};`);
    });
    lines.push(`  }`);

    lines.push('');
    lines.push(`  equals(other: ${node.name}): boolean {`);
    lines.push(`    return ${node.fields.map(f => `this.${f.name} === other.${f.name}`).join(' && ') || 'true'};`);
    lines.push(`  }`);
    lines.push(`}`);
  }

  return lines.join('\n');
}

function generateEnum(node: EnumNode, options: CodeGenOptions): string {
  const lines: string[] = [];

  if (options.includeComments) {
    lines.push(`/**`);
    lines.push(` * Enum: ${node.name}`);
    lines.push(` */`);
  }

  // Check if any variant has a payload
  const hasPayloads = node.variants.some(v => v.payload);

  if (hasPayloads) {
    // Generate discriminated union for variants with payloads
    lines.push(`export type ${node.name} =`);
    node.variants.forEach((variant, index) => {
      const separator = index === node.variants.length - 1 ? ';' : '';
      if (variant.payload) {
        lines.push(`  | { type: '${variant.name}'; value: ${mapTypeToTS(variant.payload)} }${separator}`);
      } else {
        lines.push(`  | { type: '${variant.name}' }${separator}`);
      }
    });
  } else {
    // Simple enum
    lines.push(`export enum ${node.name} {`);
    node.variants.forEach((variant, index) => {
      const comma = index < node.variants.length - 1 ? ',' : '';
      lines.push(`  ${variant.name} = '${variant.name}'${comma}`);
    });
    lines.push(`}`);
  }

  return lines.join('\n');
}

function generateAggregate(node: AggregateNode, nodes: Record<string, { node: { name: string } }>, options: CodeGenOptions): string {
  const lines: string[] = [];

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
  const memberNodes = node.memberIds.map(id => nodes[id]?.node).filter(Boolean);

  lines.push(`export interface ${node.name}Aggregate {`);
  if (rootNode) {
    lines.push(`  root: ${rootNode.name};`);
  }
  if (memberNodes.length > 0) {
    memberNodes.forEach(member => {
      if (member) {
        lines.push(`  ${member.name.toLowerCase()}s: ${member.name}[];`);
      }
    });
  }
  lines.push(`}`);

  return lines.join('\n');
}

export function generateTypeScript(context: ContextData, options: CodeGenOptions): GeneratedFile {
  const lines: string[] = [];

  if (options.includeComments) {
    lines.push(`/**`);
    lines.push(` * Generated by SketchDDD`);
    lines.push(` * Context: ${context.name}`);
    lines.push(` * Generated at: ${new Date().toISOString()}`);
    lines.push(` */`);
    lines.push('');
  }

  const nodeEntries = Object.entries(context.nodes);

  // Generate enums first
  nodeEntries
    .filter(([, data]) => data.node.kind === 'enum')
    .forEach(([, data]) => {
      lines.push(generateEnum(data.node as EnumNode, options));
      lines.push('');
    });

  // Generate value objects
  nodeEntries
    .filter(([, data]) => data.node.kind === 'value')
    .forEach(([, data]) => {
      lines.push(generateValueObject(data.node as ValueNode, options));
      lines.push('');
    });

  // Generate entities
  nodeEntries
    .filter(([, data]) => data.node.kind === 'entity')
    .forEach(([, data]) => {
      lines.push(generateEntity(data.node as EntityNode, context.morphisms, context.nodes, options));
      lines.push('');
    });

  // Generate aggregates
  nodeEntries
    .filter(([, data]) => data.node.kind === 'aggregate')
    .forEach(([, data]) => {
      lines.push(generateAggregate(data.node as AggregateNode, context.nodes, options));
      lines.push('');
    });

  const filename = `${context.name.replace(/\s+/g, '')}.ts`;

  return {
    filename,
    content: lines.join('\n').trim(),
    language: 'typescript',
  };
}
