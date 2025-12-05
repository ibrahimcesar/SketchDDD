export * from './types';
export { generateTypeScript } from './typescript';
export { generateJava } from './java';
export { generateCSharp } from './csharp';

import type { ContextData } from '@/types';
import type { CodeGenOptions, GeneratedFile, TargetLanguage } from './types';
import { generateTypeScript } from './typescript';
import { generateJava } from './java';
import { generateCSharp } from './csharp';

export function generateCode(
  context: ContextData,
  language: TargetLanguage,
  options?: Partial<CodeGenOptions>
): GeneratedFile[] {
  const defaultOptions: CodeGenOptions = {
    language,
    includeComments: true,
    generateInterfaces: true,
    generateClasses: true,
    namespace: 'Domain',
    packageName: 'com.example.domain',
    ...options,
  };

  switch (language) {
    case 'typescript':
      return [generateTypeScript(context, defaultOptions)];
    case 'java':
      return generateJava(context, defaultOptions);
    case 'csharp':
      return generateCSharp(context, defaultOptions);
    default:
      throw new Error(`Unsupported language: ${language}`);
  }
}

export function downloadGeneratedFiles(files: GeneratedFile[]): void {
  if (files.length === 1) {
    // Single file download
    const file = files[0];
    const blob = new Blob([file.content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = file.filename;
    a.click();
    URL.revokeObjectURL(url);
  } else {
    // Multiple files - create a zip (simplified: download as combined file)
    const combined = files
      .map(f => `// ============ ${f.filename} ============\n\n${f.content}`)
      .join('\n\n');
    const blob = new Blob([combined], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `generated-code.${files[0].language === 'java' ? 'java' : files[0].language === 'csharp' ? 'cs' : 'txt'}`;
    a.click();
    URL.revokeObjectURL(url);
  }
}
