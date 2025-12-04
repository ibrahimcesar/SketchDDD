import { useState } from 'react';
import { X, ArrowRight, Plus, Trash2 } from 'lucide-react';
import { useDomainStore } from '@/stores';
import type { ContextMap, ContextMapPattern, ObjectMapping } from '@/types';

interface ContextMapEditorProps {
  map: ContextMap;
  isOpen: boolean;
  onClose: () => void;
}

const patterns: { value: ContextMapPattern; label: string; description: string }[] = [
  {
    value: 'Partnership',
    label: 'Partnership',
    description: 'Two teams jointly evolve their bounded contexts',
  },
  {
    value: 'SharedKernel',
    label: 'Shared Kernel',
    description: 'Two teams share a common subset of the domain model',
  },
  {
    value: 'CustomerSupplier',
    label: 'Customer-Supplier',
    description: 'Upstream (supplier) provides what downstream (customer) needs',
  },
  {
    value: 'Conformist',
    label: 'Conformist',
    description: 'Downstream conforms to upstream model as-is',
  },
  {
    value: 'AntiCorruptionLayer',
    label: 'Anti-Corruption Layer',
    description: 'Downstream protects itself with translation layer',
  },
  {
    value: 'OpenHostService',
    label: 'Open Host Service',
    description: 'Upstream provides a well-defined protocol for integration',
  },
  {
    value: 'PublishedLanguage',
    label: 'Published Language',
    description: 'Shared language for integration (e.g., XML, JSON schemas)',
  },
  {
    value: 'SeparateWays',
    label: 'Separate Ways',
    description: 'No integration - contexts are independent',
  },
];

export function ContextMapEditor({ map, isOpen, onClose }: ContextMapEditorProps) {
  const { contexts, updateContextMap, deleteContextMap } = useDomainStore();
  const [pattern, setPattern] = useState<ContextMapPattern>(map.pattern);
  const [name, setName] = useState(map.name);
  const [mappings, setMappings] = useState<ObjectMapping[]>(map.mappings);

  const sourceContext = contexts[map.sourceContextId];
  const targetContext = contexts[map.targetContextId];

  const sourceNodes = sourceContext
    ? Object.entries(sourceContext.nodes).map(([id, { node }]) => ({
        id,
        name: node.name,
      }))
    : [];

  const targetNodes = targetContext
    ? Object.entries(targetContext.nodes).map(([id, { node }]) => ({
        id,
        name: node.name,
      }))
    : [];

  const handleSave = () => {
    updateContextMap(map.id, {
      name,
      pattern,
      mappings,
    });
    onClose();
  };

  const handleDelete = () => {
    if (confirm('Delete this context map relationship?')) {
      deleteContextMap(map.id);
      onClose();
    }
  };

  const addMapping = () => {
    setMappings([...mappings, { sourceId: '', targetId: '' }]);
  };

  const updateMapping = (index: number, updates: Partial<ObjectMapping>) => {
    setMappings(
      mappings.map((m, i) => (i === index ? { ...m, ...updates } : m))
    );
  };

  const removeMapping = (index: number) => {
    setMappings(mappings.filter((_, i) => i !== index));
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div className="absolute inset-0 bg-black/50" onClick={onClose} />

      {/* Modal */}
      <div className="relative bg-white dark:bg-slate-800 rounded-xl shadow-2xl w-full max-w-2xl mx-4 max-h-[90vh] flex flex-col overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700">
          <div>
            <h2 className="text-lg font-semibold">Edit Context Map</h2>
            <p className="text-sm text-slate-500 dark:text-slate-400">
              {sourceContext?.name || 'Unknown'} → {targetContext?.name || 'Unknown'}
            </p>
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-auto p-6 space-y-6">
          {/* Name */}
          <div>
            <label className="block text-sm font-medium mb-2">Relationship Name</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
            />
          </div>

          {/* Pattern Selection */}
          <div>
            <label className="block text-sm font-medium mb-2">Integration Pattern</label>
            <div className="grid grid-cols-2 gap-2">
              {patterns.map((p) => (
                <button
                  key={p.value}
                  onClick={() => setPattern(p.value)}
                  className={`p-3 rounded-lg border-2 text-left transition-all ${
                    pattern === p.value
                      ? 'border-primary bg-primary/10'
                      : 'border-slate-200 dark:border-slate-700 hover:border-primary/50'
                  }`}
                >
                  <div className="font-medium text-sm">{p.label}</div>
                  <div className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                    {p.description}
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* Object Mappings */}
          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="text-sm font-medium">Object Mappings</label>
              <button
                onClick={addMapping}
                className="flex items-center gap-1 text-sm text-primary hover:underline"
              >
                <Plus className="w-4 h-4" />
                Add Mapping
              </button>
            </div>
            <p className="text-xs text-slate-500 dark:text-slate-400 mb-3">
              Define how concepts in one context correspond to concepts in the other.
            </p>

            {mappings.length === 0 ? (
              <div className="p-4 bg-slate-50 dark:bg-slate-900/50 rounded-lg text-center text-sm text-slate-500 dark:text-slate-400">
                No mappings defined yet
              </div>
            ) : (
              <div className="space-y-2">
                {mappings.map((mapping, index) => (
                  <div
                    key={index}
                    className="flex items-center gap-2 p-2 bg-slate-50 dark:bg-slate-900/50 rounded-lg"
                  >
                    <select
                      value={mapping.sourceId}
                      onChange={(e) =>
                        updateMapping(index, { sourceId: e.target.value })
                      }
                      className="flex-1 px-2 py-1 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                    >
                      <option value="">Select source...</option>
                      {sourceNodes.map((n) => (
                        <option key={n.id} value={n.id}>
                          {n.name}
                        </option>
                      ))}
                    </select>

                    <ArrowRight className="w-4 h-4 text-slate-400 flex-shrink-0" />

                    <select
                      value={mapping.targetId}
                      onChange={(e) =>
                        updateMapping(index, { targetId: e.target.value })
                      }
                      className="flex-1 px-2 py-1 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                    >
                      <option value="">Select target...</option>
                      {targetNodes.map((n) => (
                        <option key={n.id} value={n.id}>
                          {n.name}
                        </option>
                      ))}
                    </select>

                    <button
                      onClick={() => removeMapping(index)}
                      className="p-1 text-red-500 hover:text-red-700"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="flex items-center justify-between px-6 py-4 border-t border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-900/50">
          <button
            onClick={handleDelete}
            className="px-4 py-2 text-red-600 hover:text-red-700 text-sm font-medium"
          >
            Delete Relationship
          </button>
          <div className="flex items-center gap-2">
            <button
              onClick={onClose}
              className="px-4 py-2 text-slate-600 dark:text-slate-400 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg text-sm font-medium"
            >
              Cancel
            </button>
            <button
              onClick={handleSave}
              className="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary-hover text-sm font-medium"
            >
              Save Changes
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
