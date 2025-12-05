import type { ContextData, EntityNode, ValueNode, EnumNode, AggregateNode } from '@/types';

export function generateMermaidClassDiagram(context: ContextData): string {
  const lines: string[] = [];

  lines.push('classDiagram');
  lines.push(`    %% Context: ${context.name}`);
  lines.push('');

  const nodeEntries = Object.entries(context.nodes);

  // Generate classes for entities
  nodeEntries
    .filter(([, data]) => data.node.kind === 'entity')
    .forEach(([, data]) => {
      const entity = data.node as EntityNode;
      lines.push(`    class ${sanitizeName(entity.name)} {`);
      lines.push(`        <<Entity>>`);
      lines.push(`        +String id`);
      entity.fields.forEach(field => {
        const optional = field.optional ? '?' : '';
        lines.push(`        +${sanitizeName(field.type)}${optional} ${field.name}`);
      });
      lines.push(`    }`);
      lines.push('');
    });

  // Generate classes for value objects
  nodeEntries
    .filter(([, data]) => data.node.kind === 'value')
    .forEach(([, data]) => {
      const value = data.node as ValueNode;
      lines.push(`    class ${sanitizeName(value.name)} {`);
      lines.push(`        <<ValueObject>>`);
      value.fields.forEach(field => {
        const optional = field.optional ? '?' : '';
        lines.push(`        +${sanitizeName(field.type)}${optional} ${field.name}`);
      });
      lines.push(`    }`);
      lines.push('');
    });

  // Generate enums
  nodeEntries
    .filter(([, data]) => data.node.kind === 'enum')
    .forEach(([, data]) => {
      const enumNode = data.node as EnumNode;
      lines.push(`    class ${sanitizeName(enumNode.name)} {`);
      lines.push(`        <<Enumeration>>`);
      enumNode.variants.forEach(variant => {
        lines.push(`        ${variant.name}`);
      });
      lines.push(`    }`);
      lines.push('');
    });

  // Generate aggregates
  nodeEntries
    .filter(([, data]) => data.node.kind === 'aggregate')
    .forEach(([, data]) => {
      const aggregate = data.node as AggregateNode;
      lines.push(`    class ${sanitizeName(aggregate.name)}Aggregate {`);
      lines.push(`        <<Aggregate>>`);
      const rootNode = context.nodes[aggregate.rootId]?.node;
      if (rootNode) {
        lines.push(`        +${sanitizeName(rootNode.name)} root`);
      }
      lines.push(`    }`);
      lines.push('');
    });

  // Generate relationships from morphisms
  context.morphisms.forEach(morphism => {
    const sourceNode = context.nodes[morphism.sourceId]?.node;
    const targetNode = context.nodes[morphism.targetId]?.node;

    if (sourceNode && targetNode) {
      const cardinality = morphism.cardinality === 'many' ? '"*"' : morphism.cardinality === 'optional' ? '"0..1"' : '"1"';
      lines.push(`    ${sanitizeName(sourceNode.name)} --> ${cardinality} ${sanitizeName(targetNode.name)} : ${morphism.name}`);
    }
  });

  // Generate aggregate containment
  nodeEntries
    .filter(([, data]) => data.node.kind === 'aggregate')
    .forEach(([, data]) => {
      const aggregate = data.node as AggregateNode;
      const rootNode = context.nodes[aggregate.rootId]?.node;
      if (rootNode) {
        lines.push(`    ${sanitizeName(aggregate.name)}Aggregate *-- ${sanitizeName(rootNode.name)}`);
      }
      aggregate.memberIds.forEach(memberId => {
        const memberNode = context.nodes[memberId]?.node;
        if (memberNode) {
          lines.push(`    ${sanitizeName(aggregate.name)}Aggregate *-- ${sanitizeName(memberNode.name)}`);
        }
      });
    });

  return lines.join('\n');
}

export function generateMermaidERDiagram(context: ContextData): string {
  const lines: string[] = [];

  lines.push('erDiagram');
  lines.push(`    %% Context: ${context.name}`);
  lines.push('');

  const nodeEntries = Object.entries(context.nodes);

  // Generate entities
  nodeEntries
    .filter(([, data]) => data.node.kind === 'entity' || data.node.kind === 'value')
    .forEach(([, data]) => {
      const node = data.node as EntityNode | ValueNode;
      lines.push(`    ${sanitizeName(node.name)} {`);
      if (data.node.kind === 'entity') {
        lines.push(`        uuid id PK`);
      }
      node.fields.forEach(field => {
        const pk = field.name.toLowerCase() === 'id' ? ' PK' : '';
        lines.push(`        ${sanitizeName(field.type)} ${field.name}${pk}`);
      });
      lines.push(`    }`);
      lines.push('');
    });

  // Generate relationships
  context.morphisms.forEach(morphism => {
    const sourceNode = context.nodes[morphism.sourceId]?.node;
    const targetNode = context.nodes[morphism.targetId]?.node;

    if (sourceNode && targetNode) {
      let relationship = '||--o{';
      if (morphism.cardinality === 'one') {
        relationship = '||--||';
      } else if (morphism.cardinality === 'optional') {
        relationship = '||--o|';
      }
      lines.push(`    ${sanitizeName(sourceNode.name)} ${relationship} ${sanitizeName(targetNode.name)} : "${morphism.name}"`);
    }
  });

  return lines.join('\n');
}

function sanitizeName(name: string): string {
  return name.replace(/[^a-zA-Z0-9_]/g, '_');
}
