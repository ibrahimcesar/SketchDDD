export { generateMermaidClassDiagram, generateMermaidERDiagram } from './mermaid';
export { generateDocumentation } from './documentation';
export type { DocumentationOptions } from './documentation';

import type { ContextData, ContextMap } from '@/types';
import { generateMermaidClassDiagram, generateMermaidERDiagram } from './mermaid';
import { generateDocumentation } from './documentation';
import type { DocumentationOptions } from './documentation';

export type DiagramFormat = 'mermaid-class' | 'mermaid-er';

export interface DiagramExport {
  format: DiagramFormat;
  content: string;
  filename: string;
}

export function generateDiagram(context: ContextData, format: DiagramFormat): DiagramExport {
  let content: string;
  let filename: string;

  switch (format) {
    case 'mermaid-class':
      content = generateMermaidClassDiagram(context);
      filename = `${context.name.replace(/\s+/g, '-')}-class-diagram.md`;
      break;
    case 'mermaid-er':
      content = generateMermaidERDiagram(context);
      filename = `${context.name.replace(/\s+/g, '-')}-er-diagram.md`;
      break;
    default:
      throw new Error(`Unsupported format: ${format}`);
  }

  return { format, content, filename };
}

export function downloadDiagram(diagram: DiagramExport): void {
  const blob = new Blob([diagram.content], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = diagram.filename;
  a.click();
  URL.revokeObjectURL(url);
}

export interface DocumentationExport {
  content: string;
  filename: string;
}

export function generateDocs(
  context: ContextData,
  contextMaps: ContextMap[] = [],
  options?: Partial<DocumentationOptions>
): DocumentationExport {
  const content = generateDocumentation(context, contextMaps, options);
  const filename = `${context.name.replace(/\s+/g, '-')}-documentation.md`;
  return { content, filename };
}

export function downloadDocumentation(doc: DocumentationExport): void {
  const blob = new Blob([doc.content], { type: 'text/markdown' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = doc.filename;
  a.click();
  URL.revokeObjectURL(url);
}
