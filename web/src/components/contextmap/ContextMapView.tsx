import { useState } from 'react';
import { ArrowLeft, Map } from 'lucide-react';
import { ContextMapCanvas } from './ContextMapCanvas';
import { ContextMapEditor } from './ContextMapEditor';
import type { ContextMap } from '@/types';

interface ContextMapViewProps {
  isOpen: boolean;
  onClose: () => void;
  onContextSelect: (contextId: string) => void;
}

export function ContextMapView({ isOpen, onClose, onContextSelect }: ContextMapViewProps) {
  const [editingMap, setEditingMap] = useState<ContextMap | null>(null);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-40 bg-slate-100 dark:bg-slate-900 flex flex-col">
      {/* Header */}
      <div className="h-12 bg-white dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between px-4">
        <div className="flex items-center gap-4">
          <button
            onClick={onClose}
            className="flex items-center gap-2 text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100"
          >
            <ArrowLeft className="w-5 h-5" />
            <span className="text-sm">Back to Editor</span>
          </button>
          <div className="w-px h-6 bg-slate-200 dark:bg-slate-700" />
          <div className="flex items-center gap-2">
            <Map className="w-5 h-5 text-purple-500" />
            <span className="font-semibold">Context Map</span>
          </div>
        </div>
        <div className="text-sm text-slate-500 dark:text-slate-400">
          Drag between contexts to create relationships • Double-click to view details
        </div>
      </div>

      {/* Canvas */}
      <div className="flex-1">
        <ContextMapCanvas
          onContextClick={(contextId) => {
            onContextSelect(contextId);
            onClose();
          }}
          onMapEdit={(map) => setEditingMap(map)}
        />
      </div>

      {/* Legend */}
      <div className="absolute bottom-4 left-4 bg-white dark:bg-slate-800 rounded-lg shadow-lg p-4">
        <div className="text-xs font-medium text-slate-500 dark:text-slate-400 mb-2">
          Integration Patterns
        </div>
        <div className="grid grid-cols-4 gap-2 text-xs">
          <div className="flex items-center gap-1">
            <span className="w-6 h-4 rounded bg-pink-500 text-white text-center text-[10px] leading-4">
              P
            </span>
            <span>Partnership</span>
          </div>
          <div className="flex items-center gap-1">
            <span className="w-6 h-4 rounded bg-blue-500 text-white text-center text-[10px] leading-4">
              SK
            </span>
            <span>Shared Kernel</span>
          </div>
          <div className="flex items-center gap-1">
            <span className="w-6 h-4 rounded bg-green-500 text-white text-center text-[10px] leading-4">
              CS
            </span>
            <span>Customer-Supplier</span>
          </div>
          <div className="flex items-center gap-1">
            <span className="w-6 h-4 rounded bg-yellow-500 text-white text-center text-[10px] leading-4">
              CF
            </span>
            <span>Conformist</span>
          </div>
          <div className="flex items-center gap-1">
            <span className="w-6 h-4 rounded bg-red-500 text-white text-center text-[10px] leading-4">
              ACL
            </span>
            <span>Anti-Corruption</span>
          </div>
          <div className="flex items-center gap-1">
            <span className="w-6 h-4 rounded bg-purple-500 text-white text-center text-[10px] leading-4">
              OHS
            </span>
            <span>Open Host</span>
          </div>
          <div className="flex items-center gap-1">
            <span className="w-6 h-4 rounded bg-indigo-500 text-white text-center text-[10px] leading-4">
              PL
            </span>
            <span>Published Lang</span>
          </div>
          <div className="flex items-center gap-1">
            <span className="w-6 h-4 rounded bg-slate-500 text-white text-center text-[10px] leading-4">
              SW
            </span>
            <span>Separate Ways</span>
          </div>
        </div>
      </div>

      {/* Map Editor Modal */}
      {editingMap && (
        <ContextMapEditor
          map={editingMap}
          isOpen={true}
          onClose={() => setEditingMap(null)}
        />
      )}
    </div>
  );
}
