import { useState } from 'react';
import { WizardModal } from './WizardModal';
import type { WizardStep } from './WizardModal';
import { useDomainStore } from '@/stores';
import { Box, Diamond, HelpCircle } from 'lucide-react';
import type { Field } from '@/types';

interface NewConceptWizardProps {
  isOpen: boolean;
  onClose: () => void;
}

export function NewConceptWizard({ isOpen, onClose }: NewConceptWizardProps) {
  const { activeContextId, addNode } = useDomainStore();

  // Step 1: Name
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');

  // Step 2: Type decision
  const [conceptType, setConceptType] = useState<'entity' | 'value' | null>(null);
  const [hasIdentity, setHasIdentity] = useState<boolean | null>(null);
  const [isImmutable, setIsImmutable] = useState<boolean | null>(null);

  // Step 3: Fields
  const [fields, setFields] = useState<Omit<Field, 'id'>[]>([
    { name: '', type: 'String', optional: false }
  ]);

  const addField = () => {
    setFields([...fields, { name: '', type: 'String', optional: false }]);
  };

  const updateField = (index: number, updates: Partial<Omit<Field, 'id'>>) => {
    setFields(fields.map((f, i) => i === index ? { ...f, ...updates } : f));
  };

  const removeField = (index: number) => {
    setFields(fields.filter((_, i) => i !== index));
  };

  // Determine type from decision tree
  const determineType = (): 'entity' | 'value' | null => {
    if (conceptType) return conceptType;
    if (hasIdentity === true) return 'entity';
    if (hasIdentity === false && isImmutable === true) return 'value';
    if (isImmutable === false) return 'entity';
    return null;
  };

  const handleFinish = () => {
    if (!activeContextId || !name) return;

    const type = determineType();
    if (!type) return;

    const validFields = fields
      .filter(f => f.name.trim())
      .map(f => ({ ...f, id: crypto.randomUUID() }));

    if (type === 'entity') {
      // Add id field for entities
      const hasIdField = validFields.some(f => f.name.toLowerCase() === 'id');
      if (!hasIdField) {
        validFields.unshift({ id: crypto.randomUUID(), name: 'id', type: 'UUID', optional: false });
      }
      addNode(activeContextId, {
        kind: 'entity',
        name,
        fields: validFields,
      }, { x: 200, y: 200 });
    } else {
      addNode(activeContextId, {
        kind: 'value',
        name,
        fields: validFields,
      }, { x: 200, y: 200 });
    }

    // Reset state
    setName('');
    setDescription('');
    setConceptType(null);
    setHasIdentity(null);
    setIsImmutable(null);
    setFields([{ name: '', type: 'String', optional: false }]);
  };

  const steps: WizardStep[] = [
    {
      title: 'Name Your Concept',
      description: 'Give your domain concept a meaningful name',
      isValid: name.trim().length > 0,
      content: (
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">Name *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., Customer, OrderItem, Address"
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
              autoFocus
            />
            <p className="text-xs text-slate-500 dark:text-slate-400 mt-1">
              Use PascalCase for concept names
            </p>
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">Description (optional)</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="What does this concept represent in your domain?"
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 h-20"
            />
          </div>
        </div>
      ),
    },
    {
      title: 'Choose Concept Type',
      description: 'Decide whether this is an Entity or Value Object',
      isValid: determineType() !== null,
      content: (
        <div className="space-y-4">
          <div className="flex items-start gap-3 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
            <HelpCircle className="w-5 h-5 text-blue-500 flex-shrink-0 mt-0.5" />
            <p className="text-sm text-blue-800 dark:text-blue-200">
              Answer these questions to help determine the right type, or choose directly below.
            </p>
          </div>

          <div className="space-y-3">
            <div className="p-3 border border-slate-200 dark:border-slate-700 rounded-lg">
              <p className="text-sm font-medium mb-2">Does this concept have a unique identity?</p>
              <div className="flex gap-2">
                <button
                  onClick={() => setHasIdentity(true)}
                  className={`px-3 py-1.5 text-sm rounded ${
                    hasIdentity === true
                      ? 'bg-primary text-white'
                      : 'bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600'
                  }`}
                >
                  Yes, it has identity
                </button>
                <button
                  onClick={() => setHasIdentity(false)}
                  className={`px-3 py-1.5 text-sm rounded ${
                    hasIdentity === false
                      ? 'bg-primary text-white'
                      : 'bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600'
                  }`}
                >
                  No identity needed
                </button>
              </div>
            </div>

            {hasIdentity === false && (
              <div className="p-3 border border-slate-200 dark:border-slate-700 rounded-lg">
                <p className="text-sm font-medium mb-2">Should it be immutable (unchangeable once created)?</p>
                <div className="flex gap-2">
                  <button
                    onClick={() => setIsImmutable(true)}
                    className={`px-3 py-1.5 text-sm rounded ${
                      isImmutable === true
                        ? 'bg-primary text-white'
                        : 'bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600'
                    }`}
                  >
                    Yes, immutable
                  </button>
                  <button
                    onClick={() => setIsImmutable(false)}
                    className={`px-3 py-1.5 text-sm rounded ${
                      isImmutable === false
                        ? 'bg-primary text-white'
                        : 'bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600'
                    }`}
                  >
                    No, it can change
                  </button>
                </div>
              </div>
            )}
          </div>

          <div className="border-t border-slate-200 dark:border-slate-700 pt-4">
            <p className="text-sm text-slate-500 dark:text-slate-400 mb-2">Or choose directly:</p>
            <div className="grid grid-cols-2 gap-3">
              <button
                onClick={() => setConceptType('entity')}
                className={`p-3 rounded-lg border-2 text-left transition-all ${
                  determineType() === 'entity'
                    ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30'
                    : 'border-slate-200 dark:border-slate-700 hover:border-blue-300'
                }`}
              >
                <Box className="w-5 h-5 text-blue-500 mb-1" />
                <div className="font-medium text-sm">Entity</div>
                <div className="text-xs text-slate-500 dark:text-slate-400">
                  Has unique identity, can change over time
                </div>
              </button>
              <button
                onClick={() => setConceptType('value')}
                className={`p-3 rounded-lg border-2 text-left transition-all ${
                  determineType() === 'value'
                    ? 'border-emerald-500 bg-emerald-50 dark:bg-emerald-900/30'
                    : 'border-slate-200 dark:border-slate-700 hover:border-emerald-300'
                }`}
              >
                <Diamond className="w-5 h-5 text-emerald-500 mb-1" />
                <div className="font-medium text-sm">Value Object</div>
                <div className="text-xs text-slate-500 dark:text-slate-400">
                  No identity, immutable, compared by value
                </div>
              </button>
            </div>
          </div>
        </div>
      ),
    },
    {
      title: 'Define Properties',
      description: 'Add the fields that describe this concept',
      isValid: fields.some(f => f.name.trim()),
      content: (
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <label className="text-sm font-medium">Fields</label>
            <button
              onClick={addField}
              className="text-sm text-primary hover:underline"
            >
              + Add Field
            </button>
          </div>

          <div className="space-y-2 max-h-[200px] overflow-y-auto">
            {fields.map((field, index) => (
              <div key={index} className="flex items-center gap-2">
                <input
                  type="text"
                  value={field.name}
                  onChange={(e) => updateField(index, { name: e.target.value })}
                  placeholder="fieldName"
                  className="flex-1 px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                />
                <select
                  value={field.type}
                  onChange={(e) => updateField(index, { type: e.target.value })}
                  className="w-28 px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                >
                  <option value="String">String</option>
                  <option value="Int">Int</option>
                  <option value="Decimal">Decimal</option>
                  <option value="Bool">Bool</option>
                  <option value="UUID">UUID</option>
                  <option value="DateTime">DateTime</option>
                  <option value="Email">Email</option>
                </select>
                <label className="flex items-center gap-1 text-xs">
                  <input
                    type="checkbox"
                    checked={field.optional}
                    onChange={(e) => updateField(index, { optional: e.target.checked })}
                  />
                  Optional
                </label>
                {fields.length > 1 && (
                  <button
                    onClick={() => removeField(index)}
                    className="text-red-500 hover:text-red-700 text-sm"
                  >
                    ×
                  </button>
                )}
              </div>
            ))}
          </div>

          {/* Preview */}
          <div className="mt-4 p-3 bg-slate-50 dark:bg-slate-900/50 rounded-lg">
            <div className="text-xs text-slate-500 dark:text-slate-400 mb-1">Preview:</div>
            <div className="font-mono text-sm">
              <span className="text-purple-600 dark:text-purple-400">
                {determineType() || 'type'}
              </span>{' '}
              <span className="text-blue-600 dark:text-blue-400">{name || 'Name'}</span>
              {' {'}<br />
              {fields.filter(f => f.name).map((f, i) => (
                <div key={i} className="ml-4">
                  {f.name}: {f.type}{f.optional ? '?' : ''}
                </div>
              ))}
              {'}'}
            </div>
          </div>
        </div>
      ),
    },
  ];

  return (
    <WizardModal
      title="Create New Concept"
      steps={steps}
      isOpen={isOpen}
      onClose={onClose}
      onFinish={handleFinish}
    />
  );
}
