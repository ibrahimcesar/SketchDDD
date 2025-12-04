import { useState, useMemo } from 'react';
import { WizardModal } from './WizardModal';
import type { WizardStep } from './WizardModal';
import { useDomainStore } from '@/stores';
import { ArrowRight } from 'lucide-react';
import type { Morphism } from '@/types';

interface ConnectConceptsWizardProps {
  isOpen: boolean;
  onClose: () => void;
}

export function ConnectConceptsWizard({ isOpen, onClose }: ConnectConceptsWizardProps) {
  const { activeContextId, contexts, addMorphism } = useDomainStore();
  const activeContext = activeContextId ? contexts[activeContextId] : null;

  // Step 1: Select source and target
  const [sourceId, setSourceId] = useState<string>('');
  const [targetId, setTargetId] = useState<string>('');

  // Step 2: Relationship type
  const [cardinality, setCardinality] = useState<Morphism['cardinality']>('one');

  // Step 3: Name
  const [name, setName] = useState('');

  const nodes = useMemo(() => {
    if (!activeContext) return [];
    return Object.entries(activeContext.nodes)
      .filter(([_, { node }]) => node.kind !== 'aggregate')
      .map(([id, { node }]) => ({ id, name: node.name, kind: node.kind }));
  }, [activeContext]);

  const sourceName = nodes.find(n => n.id === sourceId)?.name || '';
  const targetName = nodes.find(n => n.id === targetId)?.name || '';

  // Auto-generate name suggestion
  const suggestedName = useMemo(() => {
    if (!sourceName || !targetName) return '';
    const target = targetName.charAt(0).toLowerCase() + targetName.slice(1);
    if (cardinality === 'many') {
      return `${target}s`;
    }
    return target;
  }, [sourceName, targetName, cardinality]);

  const handleFinish = () => {
    if (!activeContextId || !sourceId || !targetId || !name) return;

    addMorphism(activeContextId, {
      name,
      sourceId,
      targetId,
      cardinality,
    });

    // Reset
    setSourceId('');
    setTargetId('');
    setCardinality('one');
    setName('');
  };

  const steps: WizardStep[] = [
    {
      title: 'Select Concepts',
      description: 'Choose which concepts to connect',
      isValid: sourceId !== '' && targetId !== '' && sourceId !== targetId,
      content: (
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-2">Source (from)</label>
            <select
              value={sourceId}
              onChange={(e) => setSourceId(e.target.value)}
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
            >
              <option value="">Select source concept...</option>
              {nodes.map((node) => (
                <option key={node.id} value={node.id}>
                  {node.name} ({node.kind})
                </option>
              ))}
            </select>
          </div>

          <div className="flex justify-center">
            <ArrowRight className="w-6 h-6 text-slate-400" />
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">Target (to)</label>
            <select
              value={targetId}
              onChange={(e) => setTargetId(e.target.value)}
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
            >
              <option value="">Select target concept...</option>
              {nodes.map((node) => (
                <option key={node.id} value={node.id} disabled={node.id === sourceId}>
                  {node.name} ({node.kind})
                </option>
              ))}
            </select>
          </div>

          {sourceId && targetId && sourceId === targetId && (
            <p className="text-sm text-red-500">Source and target must be different</p>
          )}
        </div>
      ),
    },
    {
      title: 'Relationship Type',
      description: 'Define the cardinality of the relationship',
      isValid: true,
      content: (
        <div className="space-y-4">
          <p className="text-sm text-slate-600 dark:text-slate-400">
            How many <strong>{targetName || 'targets'}</strong> can a{' '}
            <strong>{sourceName || 'source'}</strong> have?
          </p>

          <div className="space-y-2">
            <button
              onClick={() => setCardinality('one')}
              className={`w-full p-4 rounded-lg border-2 text-left transition-all ${
                cardinality === 'one'
                  ? 'border-primary bg-primary/10'
                  : 'border-slate-200 dark:border-slate-700 hover:border-primary/50'
              }`}
            >
              <div className="font-medium">Exactly One</div>
              <div className="text-sm text-slate-500 dark:text-slate-400">
                {sourceName || 'Source'} → {targetName || 'Target'}
              </div>
              <div className="text-xs text-slate-400 dark:text-slate-500 mt-1">
                Example: An Order has exactly one Customer
              </div>
            </button>

            <button
              onClick={() => setCardinality('optional')}
              className={`w-full p-4 rounded-lg border-2 text-left transition-all ${
                cardinality === 'optional'
                  ? 'border-primary bg-primary/10'
                  : 'border-slate-200 dark:border-slate-700 hover:border-primary/50'
              }`}
            >
              <div className="font-medium">Zero or One (Optional)</div>
              <div className="text-sm text-slate-500 dark:text-slate-400">
                {sourceName || 'Source'} → {targetName || 'Target'}?
              </div>
              <div className="text-xs text-slate-400 dark:text-slate-500 mt-1">
                Example: A User may have a ProfilePicture
              </div>
            </button>

            <button
              onClick={() => setCardinality('many')}
              className={`w-full p-4 rounded-lg border-2 text-left transition-all ${
                cardinality === 'many'
                  ? 'border-primary bg-primary/10'
                  : 'border-slate-200 dark:border-slate-700 hover:border-primary/50'
              }`}
            >
              <div className="font-medium">Many</div>
              <div className="text-sm text-slate-500 dark:text-slate-400">
                {sourceName || 'Source'} → [{targetName || 'Target'}]
              </div>
              <div className="text-xs text-slate-400 dark:text-slate-500 mt-1">
                Example: An Order has many LineItems
              </div>
            </button>
          </div>
        </div>
      ),
    },
    {
      title: 'Name the Relationship',
      description: 'Give the relationship a meaningful name',
      isValid: name.trim().length > 0,
      content: (
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">Relationship Name *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder={suggestedName || 'e.g., customer, items, address'}
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
              autoFocus
            />
            {suggestedName && !name && (
              <button
                onClick={() => setName(suggestedName)}
                className="text-xs text-primary hover:underline mt-1"
              >
                Use suggested: {suggestedName}
              </button>
            )}
          </div>

          {/* Preview */}
          <div className="p-4 bg-slate-50 dark:bg-slate-900/50 rounded-lg">
            <div className="text-xs text-slate-500 dark:text-slate-400 mb-2">Preview:</div>
            <div className="font-mono text-sm">
              <span className="text-blue-600 dark:text-blue-400">{name || '...'}</span>
              :{' '}
              <span className="text-purple-600 dark:text-purple-400">{sourceName || 'Source'}</span>
              {' → '}
              <span className="text-purple-600 dark:text-purple-400">
                {cardinality === 'many' && '['}
                {targetName || 'Target'}
                {cardinality === 'many' && ']'}
                {cardinality === 'optional' && '?'}
              </span>
            </div>
          </div>
        </div>
      ),
    },
  ];

  return (
    <WizardModal
      title="Connect Concepts"
      steps={steps}
      isOpen={isOpen}
      onClose={onClose}
      onFinish={handleFinish}
    />
  );
}
