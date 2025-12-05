import { useState, useMemo } from 'react';
import { X, Download, Copy, Check, Code } from 'lucide-react';
import { useDomainStore } from '@/stores';
import { generateCode, downloadGeneratedFiles } from '@/utils/codegen';
import type { TargetLanguage, GeneratedFile } from '@/utils/codegen';

interface CodeGenPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

const languages: { id: TargetLanguage; name: string; icon: string }[] = [
  { id: 'typescript', name: 'TypeScript', icon: 'TS' },
  { id: 'java', name: 'Java', icon: 'JV' },
  { id: 'csharp', name: 'C#', icon: 'C#' },
];

export function CodeGenPanel({ isOpen, onClose }: CodeGenPanelProps) {
  const { contexts, activeContextId } = useDomainStore();
  const [selectedLanguage, setSelectedLanguage] = useState<TargetLanguage>('typescript');
  const [selectedFileIndex, setSelectedFileIndex] = useState(0);
  const [copied, setCopied] = useState(false);
  const [includeComments, setIncludeComments] = useState(true);
  const [generateInterfaces, setGenerateInterfaces] = useState(true);
  const [generateClasses, setGenerateClasses] = useState(true);
  const [namespace, setNamespace] = useState('Domain');
  const [packageName, setPackageName] = useState('com.example.domain');

  const activeContext = activeContextId ? contexts[activeContextId] : null;

  const generatedFiles: GeneratedFile[] = useMemo(() => {
    if (!activeContext) return [];

    try {
      return generateCode(activeContext, selectedLanguage, {
        includeComments,
        generateInterfaces,
        generateClasses,
        namespace,
        packageName,
      });
    } catch (error) {
      console.error('Code generation failed:', error);
      return [];
    }
  }, [activeContext, selectedLanguage, includeComments, generateInterfaces, generateClasses, namespace, packageName]);

  const selectedFile = generatedFiles[selectedFileIndex] || null;

  const handleCopy = async () => {
    if (!selectedFile) return;

    await navigator.clipboard.writeText(selectedFile.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownload = () => {
    if (generatedFiles.length === 0) return;
    downloadGeneratedFiles(generatedFiles);
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-5xl h-[80vh] flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-3">
            <Code className="w-5 h-5 text-primary" />
            <h2 className="text-lg font-semibold">Generate Code</h2>
            {activeContext && (
              <span className="text-sm text-slate-500 dark:text-slate-400">
                — {activeContext.name}
              </span>
            )}
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {!activeContext ? (
          <div className="flex-1 flex items-center justify-center text-slate-500 dark:text-slate-400">
            <p>Select a bounded context to generate code</p>
          </div>
        ) : (
          <div className="flex-1 flex overflow-hidden">
            {/* Sidebar */}
            <div className="w-64 border-r border-slate-200 dark:border-slate-700 p-4 overflow-y-auto">
              {/* Language Selection */}
              <div className="mb-6">
                <label className="block text-sm font-medium mb-2">Target Language</label>
                <div className="space-y-1">
                  {languages.map(lang => (
                    <button
                      key={lang.id}
                      onClick={() => {
                        setSelectedLanguage(lang.id);
                        setSelectedFileIndex(0);
                      }}
                      className={`w-full flex items-center gap-3 px-3 py-2 rounded text-left ${
                        selectedLanguage === lang.id
                          ? 'bg-primary text-white'
                          : 'hover:bg-slate-100 dark:hover:bg-slate-700'
                      }`}
                    >
                      <span
                        className={`w-8 h-8 rounded flex items-center justify-center text-xs font-bold ${
                          selectedLanguage === lang.id
                            ? 'bg-white/20'
                            : 'bg-slate-200 dark:bg-slate-600'
                        }`}
                      >
                        {lang.icon}
                      </span>
                      <span>{lang.name}</span>
                    </button>
                  ))}
                </div>
              </div>

              {/* Options */}
              <div className="mb-6">
                <label className="block text-sm font-medium mb-2">Options</label>
                <div className="space-y-2">
                  <label className="flex items-center gap-2 text-sm">
                    <input
                      type="checkbox"
                      checked={includeComments}
                      onChange={(e) => setIncludeComments(e.target.checked)}
                      className="rounded"
                    />
                    Include comments
                  </label>
                  {selectedLanguage === 'typescript' && (
                    <>
                      <label className="flex items-center gap-2 text-sm">
                        <input
                          type="checkbox"
                          checked={generateInterfaces}
                          onChange={(e) => setGenerateInterfaces(e.target.checked)}
                          className="rounded"
                        />
                        Generate interfaces
                      </label>
                      <label className="flex items-center gap-2 text-sm">
                        <input
                          type="checkbox"
                          checked={generateClasses}
                          onChange={(e) => setGenerateClasses(e.target.checked)}
                          className="rounded"
                        />
                        Generate classes
                      </label>
                    </>
                  )}
                </div>
              </div>

              {/* Namespace/Package */}
              <div className="mb-6">
                {selectedLanguage === 'java' ? (
                  <div>
                    <label className="block text-sm font-medium mb-1">Package Name</label>
                    <input
                      type="text"
                      value={packageName}
                      onChange={(e) => setPackageName(e.target.value)}
                      className="w-full px-2 py-1 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                    />
                  </div>
                ) : selectedLanguage === 'csharp' ? (
                  <div>
                    <label className="block text-sm font-medium mb-1">Namespace</label>
                    <input
                      type="text"
                      value={namespace}
                      onChange={(e) => setNamespace(e.target.value)}
                      className="w-full px-2 py-1 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                    />
                  </div>
                ) : null}
              </div>

              {/* File List */}
              {generatedFiles.length > 1 && (
                <div>
                  <label className="block text-sm font-medium mb-2">Files</label>
                  <div className="space-y-1">
                    {generatedFiles.map((file, index) => (
                      <button
                        key={file.filename}
                        onClick={() => setSelectedFileIndex(index)}
                        className={`w-full text-left px-3 py-2 rounded text-sm truncate ${
                          selectedFileIndex === index
                            ? 'bg-slate-200 dark:bg-slate-600'
                            : 'hover:bg-slate-100 dark:hover:bg-slate-700'
                        }`}
                      >
                        {file.filename}
                      </button>
                    ))}
                  </div>
                </div>
              )}
            </div>

            {/* Code Preview */}
            <div className="flex-1 flex flex-col overflow-hidden">
              {/* Toolbar */}
              <div className="flex items-center justify-between px-4 py-2 border-b border-slate-200 dark:border-slate-700">
                <span className="text-sm font-medium text-slate-600 dark:text-slate-400">
                  {selectedFile?.filename || 'No file selected'}
                </span>
                <div className="flex items-center gap-2">
                  <button
                    onClick={handleCopy}
                    className="flex items-center gap-1 px-3 py-1 text-sm rounded border border-slate-300 dark:border-slate-600 hover:bg-slate-100 dark:hover:bg-slate-700"
                  >
                    {copied ? <Check className="w-4 h-4 text-green-500" /> : <Copy className="w-4 h-4" />}
                    {copied ? 'Copied!' : 'Copy'}
                  </button>
                  <button
                    onClick={handleDownload}
                    className="flex items-center gap-1 px-3 py-1 text-sm rounded bg-primary text-white hover:bg-primary-hover"
                  >
                    <Download className="w-4 h-4" />
                    Download
                  </button>
                </div>
              </div>

              {/* Code Content */}
              <div className="flex-1 overflow-auto bg-slate-900 p-4">
                {selectedFile ? (
                  <pre className="text-sm text-slate-100 font-mono whitespace-pre-wrap">
                    <code>{selectedFile.content}</code>
                  </pre>
                ) : (
                  <p className="text-slate-400">No code generated</p>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
