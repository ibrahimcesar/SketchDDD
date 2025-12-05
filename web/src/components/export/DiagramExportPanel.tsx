import { useState, useMemo } from 'react';
import { X, Download, Copy, Check, FileImage, FileText, BookOpen } from 'lucide-react';
import { useDomainStore } from '@/stores';
import { generateDiagram, downloadDiagram, generateDocs, downloadDocumentation } from '@/utils/export';
import type { DiagramFormat } from '@/utils/export';

interface DiagramExportPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

type ExportTab = 'diagram' | 'documentation';

const diagramFormats: { id: DiagramFormat; name: string; description: string }[] = [
  { id: 'mermaid-class', name: 'Mermaid Class Diagram', description: 'UML-style class diagram' },
  { id: 'mermaid-er', name: 'Mermaid ER Diagram', description: 'Entity-relationship diagram' },
];

export function DiagramExportPanel({ isOpen, onClose }: DiagramExportPanelProps) {
  const { contexts, activeContextId, contextMaps } = useDomainStore();
  const [activeTab, setActiveTab] = useState<ExportTab>('diagram');
  const [selectedFormat, setSelectedFormat] = useState<DiagramFormat>('mermaid-class');
  const [copied, setCopied] = useState(false);

  // Documentation options
  const [includeTOC, setIncludeTOC] = useState(true);
  const [includeRelationships, setIncludeRelationships] = useState(true);
  const [includeContextMaps, setIncludeContextMaps] = useState(true);

  const activeContext = activeContextId ? contexts[activeContextId] : null;

  const diagram = useMemo(() => {
    if (!activeContext || activeTab !== 'diagram') return null;

    try {
      return generateDiagram(activeContext, selectedFormat);
    } catch (error) {
      console.error('Diagram generation failed:', error);
      return null;
    }
  }, [activeContext, selectedFormat, activeTab]);

  const documentation = useMemo(() => {
    if (!activeContext || activeTab !== 'documentation') return null;

    try {
      return generateDocs(activeContext, contextMaps, {
        includeTableOfContents: includeTOC,
        includeRelationships,
        includeContextMaps,
      });
    } catch (error) {
      console.error('Documentation generation failed:', error);
      return null;
    }
  }, [activeContext, contextMaps, activeTab, includeTOC, includeRelationships, includeContextMaps]);

  const currentContent = activeTab === 'diagram' ? diagram?.content : documentation?.content;
  const currentFilename = activeTab === 'diagram' ? diagram?.filename : documentation?.filename;

  const handleCopy = async () => {
    if (!currentContent) return;

    await navigator.clipboard.writeText(currentContent);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleDownload = () => {
    if (activeTab === 'diagram' && diagram) {
      downloadDiagram(diagram);
    } else if (activeTab === 'documentation' && documentation) {
      downloadDocumentation(documentation);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-4xl h-[70vh] flex flex-col">
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-3">
            {activeTab === 'diagram' ? (
              <FileImage className="w-5 h-5 text-primary" />
            ) : (
              <BookOpen className="w-5 h-5 text-primary" />
            )}
            <h2 className="text-lg font-semibold">Export</h2>
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

        {/* Tabs */}
        <div className="flex border-b border-slate-200 dark:border-slate-700">
          <button
            onClick={() => setActiveTab('diagram')}
            className={`flex items-center gap-2 px-4 py-2 text-sm font-medium border-b-2 -mb-px ${
              activeTab === 'diagram'
                ? 'border-primary text-primary'
                : 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'
            }`}
          >
            <FileImage className="w-4 h-4" />
            Diagrams
          </button>
          <button
            onClick={() => setActiveTab('documentation')}
            className={`flex items-center gap-2 px-4 py-2 text-sm font-medium border-b-2 -mb-px ${
              activeTab === 'documentation'
                ? 'border-primary text-primary'
                : 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'
            }`}
          >
            <BookOpen className="w-4 h-4" />
            Documentation
          </button>
        </div>

        {!activeContext ? (
          <div className="flex-1 flex items-center justify-center text-slate-500 dark:text-slate-400">
            <p>Select a bounded context to export</p>
          </div>
        ) : (
          <div className="flex-1 flex overflow-hidden">
            {/* Sidebar */}
            <div className="w-64 border-r border-slate-200 dark:border-slate-700 p-4 overflow-y-auto">
              {activeTab === 'diagram' ? (
                <>
                  <label className="block text-sm font-medium mb-3">Diagram Format</label>
                  <div className="space-y-2">
                    {diagramFormats.map(format => (
                      <button
                        key={format.id}
                        onClick={() => setSelectedFormat(format.id)}
                        className={`w-full text-left p-3 rounded-lg border ${
                          selectedFormat === format.id
                            ? 'border-primary bg-primary/10'
                            : 'border-slate-200 dark:border-slate-600 hover:bg-slate-50 dark:hover:bg-slate-700'
                        }`}
                      >
                        <div className="font-medium text-sm">{format.name}</div>
                        <div className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                          {format.description}
                        </div>
                      </button>
                    ))}
                  </div>

                  <div className="mt-6 p-3 bg-slate-100 dark:bg-slate-700 rounded-lg">
                    <div className="flex items-center gap-2 text-sm font-medium mb-2">
                      <FileText className="w-4 h-4" />
                      Mermaid Diagrams
                    </div>
                    <p className="text-xs text-slate-600 dark:text-slate-400">
                      Paste the generated code into any Mermaid-compatible editor like:
                    </p>
                    <ul className="text-xs text-slate-600 dark:text-slate-400 mt-2 space-y-1">
                      <li>• GitHub Markdown</li>
                      <li>• Notion</li>
                      <li>• Mermaid Live Editor</li>
                      <li>• VS Code with Mermaid extension</li>
                    </ul>
                  </div>
                </>
              ) : (
                <>
                  <label className="block text-sm font-medium mb-3">Options</label>
                  <div className="space-y-3">
                    <label className="flex items-center gap-2 text-sm">
                      <input
                        type="checkbox"
                        checked={includeTOC}
                        onChange={(e) => setIncludeTOC(e.target.checked)}
                        className="rounded"
                      />
                      Include Table of Contents
                    </label>
                    <label className="flex items-center gap-2 text-sm">
                      <input
                        type="checkbox"
                        checked={includeRelationships}
                        onChange={(e) => setIncludeRelationships(e.target.checked)}
                        className="rounded"
                      />
                      Include Relationships
                    </label>
                    <label className="flex items-center gap-2 text-sm">
                      <input
                        type="checkbox"
                        checked={includeContextMaps}
                        onChange={(e) => setIncludeContextMaps(e.target.checked)}
                        className="rounded"
                      />
                      Include Context Maps
                    </label>
                  </div>

                  <div className="mt-6 p-3 bg-slate-100 dark:bg-slate-700 rounded-lg">
                    <div className="flex items-center gap-2 text-sm font-medium mb-2">
                      <BookOpen className="w-4 h-4" />
                      Markdown Documentation
                    </div>
                    <p className="text-xs text-slate-600 dark:text-slate-400">
                      Generated documentation includes:
                    </p>
                    <ul className="text-xs text-slate-600 dark:text-slate-400 mt-2 space-y-1">
                      <li>• Entity descriptions</li>
                      <li>• Value object definitions</li>
                      <li>• Enumeration variants</li>
                      <li>• Aggregate structures</li>
                      <li>• Field tables</li>
                    </ul>
                  </div>
                </>
              )}
            </div>

            {/* Preview */}
            <div className="flex-1 flex flex-col overflow-hidden">
              {/* Toolbar */}
              <div className="flex items-center justify-between px-4 py-2 border-b border-slate-200 dark:border-slate-700">
                <span className="text-sm font-medium text-slate-600 dark:text-slate-400">
                  {currentFilename || 'No content'}
                </span>
                <div className="flex items-center gap-2">
                  <button
                    onClick={handleCopy}
                    disabled={!currentContent}
                    className="flex items-center gap-1 px-3 py-1 text-sm rounded border border-slate-300 dark:border-slate-600 hover:bg-slate-100 dark:hover:bg-slate-700 disabled:opacity-50"
                  >
                    {copied ? <Check className="w-4 h-4 text-green-500" /> : <Copy className="w-4 h-4" />}
                    {copied ? 'Copied!' : 'Copy'}
                  </button>
                  <button
                    onClick={handleDownload}
                    disabled={!currentContent}
                    className="flex items-center gap-1 px-3 py-1 text-sm rounded bg-primary text-white hover:bg-primary-hover disabled:opacity-50"
                  >
                    <Download className="w-4 h-4" />
                    Download
                  </button>
                </div>
              </div>

              {/* Content */}
              <div className="flex-1 overflow-auto bg-slate-900 p-4">
                {currentContent ? (
                  <pre className="text-sm text-slate-100 font-mono whitespace-pre-wrap">
                    <code>{currentContent}</code>
                  </pre>
                ) : (
                  <p className="text-slate-400">No content generated</p>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
