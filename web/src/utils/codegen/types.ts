export type TargetLanguage = 'typescript' | 'java' | 'csharp';

export interface CodeGenOptions {
  language: TargetLanguage;
  includeComments: boolean;
  generateInterfaces: boolean;
  generateClasses: boolean;
  namespace?: string;
  packageName?: string;
}

export interface GeneratedFile {
  filename: string;
  content: string;
  language: TargetLanguage;
}
