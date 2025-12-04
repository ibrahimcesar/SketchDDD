import { useState, useMemo } from 'react';
import { WizardModal } from './WizardModal';
import type { WizardStep } from './WizardModal';
import { useDomainStore } from '@/stores';
import { Layers, Check } from 'lucide-react';

interface AggregateWizardProps {
  isOpen: boolean;
  onClose: () => void;
}

export function AggregateWizard({ isOpen, onClose }: AggregateWizardProps) {
  const { activeContextId, contexts, addNode } = useDomainStore();
  const activeContext = activeContextId ? contexts[activeContextId] : null;

  // Step 1: Name and root
  const [name, setName] = useState('');
  const [rootId, setRootId] = useState<string>('');

  // Step 2: Members
  const [memberIds, setMemberIds] = useState<string[]>([]);

  // Step 3: Invariants
  const [invariants, setInvariants] = useState<string[]>(['']);

  const entities = useMemo(() => {
    if (!activeContext) return [];
    return Object.entries(activeContext.nodes)
      .filter(([_, { node }]) => node.kind === 'entity')
      .map(([id, { node }]) => ({ id, name: node.name }));
  }, [activeContext]);

  const rootEntity = entities.find(e => e.id === rootId);

  const toggleMember = (id: string) => {
    if (id === rootId) return; // Can't add root as member
    setMemberIds(prev =>
      prev.includes(id)
        ? prev.filter(m => m !== id)
        : [...prev, id]
    );
  };

  const addInvariant = () => {
    setInvariants([...invariants, '']);
  };

  const updateInvariant = (index: number, value: string) => {
    setInvariants(invariants.map((inv, i) => i === index ? value : inv));
  };

  const removeInvariant = (index: number) => {
    setInvariants(invariants.filter((_, i) => i !== index));
  };

  const handleFinish = () => {
    if (!activeContextId || !name || !rootId) return;

    const validInvariants = invariants.filter(inv => inv.trim());

    addNode(activeContextId, {
      kind: 'aggregate',
      name,
      rootId,
      memberIds,
      invariants: validInvariants,
    }, { x: 300, y: 200 });

    // Reset
    setName('');
    setRootId('');
    setMemberIds([]);
    setInvariants(['']);
  };

  const steps: WizardStep[] = [
    {
      title: 'Name & Root Entity',
      description: 'Define the aggregate and its root',
      isValid: name.trim().length > 0 && rootId !== '',
      content: (
        <div className="space-y-4">
          <div className="p-3 bg-purple-50 dark:bg-purple-900/20 rounded-lg border border-purple-200 dark:border-purple-800">
            <div className="flex items-start gap-2">
              <Layers className="w-5 h-5 text-purple-500 flex-shrink-0 mt-0.5" />
              <div className="text-sm text-purple-800 dark:text-purple-200">
                <strong>What is an Aggregate?</strong><br />
                An aggregate is a cluster of domain objects treated as a single unit.
                The root entity is the only entry point for external references.
              </div>
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">Aggregate Name *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., OrderAggregate, CartAggregate"
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
              autoFocus
            />
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">Root Entity *</label>
            {entities.length === 0 ? (
              <p className="text-sm text-slate-500 dark:text-slate-400 italic">
                No entities available. Create an entity first.
              </p>
            ) : (
              <div className="space-y-2">
                {entities.map((entity) => (
                  <button
                    key={entity.id}
                    onClick={() => {
                      setRootId(entity.id);
                      if (!name) setName(`${entity.name}Aggregate`);
                    }}
                    className={`w-full p-3 rounded-lg border-2 text-left transition-all ${
                      rootId === entity.id
                        ? 'border-primary bg-primary/10'
                        : 'border-slate-200 dark:border-slate-700 hover:border-primary/50'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-medium">{entity.name}</span>
                      {rootId === entity.id && <Check className="w-4 h-4 text-primary" />}
                    </div>
                  </button>
                ))}
              </div>
            )}
          </div>
        </div>
      ),
    },
    {
      title: 'Select Members',
      description: 'Choose entities contained within this aggregate',
      isValid: true, // Members are optional
      content: (
        <div className="space-y-4">
          <p className="text-sm text-slate-600 dark:text-slate-400">
            Select which entities should be part of this aggregate.
            These entities can only be accessed through the root.
          </p>

          {entities.filter(e => e.id !== rootId).length === 0 ? (
            <div className="p-4 bg-slate-50 dark:bg-slate-900/50 rounded-lg text-center">
              <p className="text-sm text-slate-500 dark:text-slate-400">
                No other entities available to add as members.
              </p>
            </div>
          ) : (
            <div className="space-y-2">
              {entities
                .filter(e => e.id !== rootId)
                .map((entity) => (
                  <button
                    key={entity.id}
                    onClick={() => toggleMember(entity.id)}
                    className={`w-full p-3 rounded-lg border-2 text-left transition-all ${
                      memberIds.includes(entity.id)
                        ? 'border-primary bg-primary/10'
                        : 'border-slate-200 dark:border-slate-700 hover:border-primary/50'
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="font-medium">{entity.name}</span>
                      {memberIds.includes(entity.id) && (
                        <Check className="w-4 h-4 text-primary" />
                      )}
                    </div>
                  </button>
                ))}
            </div>
          )}

          {/* Preview */}
          <div className="p-3 bg-slate-50 dark:bg-slate-900/50 rounded-lg">
            <div className="text-xs text-slate-500 dark:text-slate-400 mb-1">
              Aggregate Structure:
            </div>
            <div className="text-sm">
              <span className="font-medium text-red-600 dark:text-red-400">
                {rootEntity?.name || 'Root'}
              </span>
              {' (root)'}
              {memberIds.length > 0 && (
                <div className="ml-4 mt-1 text-slate-600 dark:text-slate-400">
                  Contains: {memberIds.map(id =>
                    entities.find(e => e.id === id)?.name
                  ).join(', ')}
                </div>
              )}
            </div>
          </div>
        </div>
      ),
    },
    {
      title: 'Define Invariants',
      description: 'Specify the business rules that must always be true',
      isValid: true, // Invariants are optional
      content: (
        <div className="space-y-4">
          <p className="text-sm text-slate-600 dark:text-slate-400">
            Invariants are business rules that the aggregate must enforce.
            They are conditions that must always be true.
          </p>

          <div className="space-y-2">
            {invariants.map((invariant, index) => (
              <div key={index} className="flex items-center gap-2">
                <input
                  type="text"
                  value={invariant}
                  onChange={(e) => updateInvariant(index, e.target.value)}
                  placeholder="e.g., totalPrice = sum(items.price)"
                  className="flex-1 px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
                />
                {invariants.length > 1 && (
                  <button
                    onClick={() => removeInvariant(index)}
                    className="text-red-500 hover:text-red-700 px-2"
                  >
                    ×
                  </button>
                )}
              </div>
            ))}
          </div>

          <button
            onClick={addInvariant}
            className="text-sm text-primary hover:underline"
          >
            + Add Invariant
          </button>

          {/* Examples */}
          <div className="p-3 bg-slate-50 dark:bg-slate-900/50 rounded-lg">
            <div className="text-xs text-slate-500 dark:text-slate-400 mb-2">
              Example invariants:
            </div>
            <ul className="text-xs text-slate-600 dark:text-slate-400 space-y-1">
              <li>• items.quantity {'>'} 0</li>
              <li>• totalPrice = sum(items.price)</li>
              <li>• status != "CANCELLED" OR refundIssued = true</li>
            </ul>
          </div>
        </div>
      ),
    },
  ];

  return (
    <WizardModal
      title="Create Aggregate"
      steps={steps}
      isOpen={isOpen}
      onClose={onClose}
      onFinish={handleFinish}
    />
  );
}
