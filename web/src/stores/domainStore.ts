import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type {
  DomainNode,
  Morphism,
  ContextData,
  ContextMap,
  ValidationResult,
  Field,
  EnumVariant,
} from '@/types';
import { parse as wasmParse, isWasmInitialized } from '@/wasm';
import type { ParseResult } from '@/wasm';

interface HistoryState {
  past: DomainState[];
  future: DomainState[];
}

interface DomainState {
  // Current context being edited
  activeContextId: string | null;

  // All contexts in the model
  contexts: Record<string, ContextData>;

  // Context maps between bounded contexts
  contextMaps: ContextMap[];

  // Selection state
  selectedNodeIds: string[];
  selectedEdgeIds: string[];

  // Validation state
  validationResult: ValidationResult | null;

  // UI state
  isPaletteOpen: boolean;
  isPropertiesPanelOpen: boolean;
}

interface DomainActions {
  // Context management
  createContext: (name: string) => string;
  deleteContext: (contextId: string) => void;
  setActiveContext: (contextId: string | null) => void;
  renameContext: (contextId: string, name: string) => void;

  // Node management
  addNode: (contextId: string, node: DomainNode, position: { x: number; y: number }) => string;
  updateNode: (contextId: string, nodeId: string, updates: Partial<DomainNode>) => void;
  deleteNode: (contextId: string, nodeId: string) => void;
  moveNode: (contextId: string, nodeId: string, position: { x: number; y: number }) => void;

  // Field management (for entities and values)
  addField: (contextId: string, nodeId: string, field: Omit<Field, 'id'>) => void;
  updateField: (contextId: string, nodeId: string, fieldId: string, updates: Partial<Field>) => void;
  deleteField: (contextId: string, nodeId: string, fieldId: string) => void;

  // Enum variant management
  addVariant: (contextId: string, nodeId: string, variant: Omit<EnumVariant, 'id'>) => void;
  updateVariant: (contextId: string, nodeId: string, variantId: string, updates: Partial<EnumVariant>) => void;
  deleteVariant: (contextId: string, nodeId: string, variantId: string) => void;

  // Morphism management
  addMorphism: (contextId: string, morphism: Omit<Morphism, 'id'>) => string;
  updateMorphism: (contextId: string, morphismId: string, updates: Partial<Morphism>) => void;
  deleteMorphism: (contextId: string, morphismId: string) => void;

  // Context map management
  addContextMap: (map: Omit<ContextMap, 'id'>) => string;
  updateContextMap: (mapId: string, updates: Partial<ContextMap>) => void;
  deleteContextMap: (mapId: string) => void;

  // Selection
  setSelectedNodes: (nodeIds: string[]) => void;
  setSelectedEdges: (edgeIds: string[]) => void;
  clearSelection: () => void;

  // Validation
  setValidationResult: (result: ValidationResult | null) => void;

  // UI state
  togglePalette: () => void;
  togglePropertiesPanel: () => void;

  // History (undo/redo)
  undo: () => void;
  redo: () => void;
  canUndo: () => boolean;
  canRedo: () => boolean;

  // Import/Export
  exportModel: () => string;
  importModel: (sddd: string) => void;
  reset: () => void;
}

const generateId = () => crypto.randomUUID();

const initialState: DomainState = {
  activeContextId: null,
  contexts: {},
  contextMaps: [],
  selectedNodeIds: [],
  selectedEdgeIds: [],
  validationResult: null,
  isPaletteOpen: true,
  isPropertiesPanelOpen: true,
};

// History storage (separate from main state to avoid persistence issues)
const history: HistoryState = {
  past: [],
  future: [],
};

const MAX_HISTORY = 50;

const saveToHistory = (state: DomainState) => {
  history.past.push(JSON.parse(JSON.stringify(state)));
  if (history.past.length > MAX_HISTORY) {
    history.past.shift();
  }
  history.future = [];
};

export const useDomainStore = create<DomainState & DomainActions>()(
  devtools(
    persist(
      immer((set, get) => ({
        ...initialState,

        // Context management
        createContext: (name) => {
          const id = generateId();
          set((state) => {
            saveToHistory(state);
            state.contexts[id] = {
              id,
              name,
              nodes: {},
              morphisms: [],
            };
            state.activeContextId = id;
          });
          return id;
        },

        deleteContext: (contextId) => {
          set((state) => {
            saveToHistory(state);
            delete state.contexts[contextId];
            state.contextMaps = state.contextMaps.filter(
              (m) => m.sourceContextId !== contextId && m.targetContextId !== contextId
            );
            if (state.activeContextId === contextId) {
              const contextIds = Object.keys(state.contexts);
              state.activeContextId = contextIds.length > 0 ? contextIds[0] : null;
            }
          });
        },

        setActiveContext: (contextId) => {
          set((state) => {
            state.activeContextId = contextId;
            state.selectedNodeIds = [];
            state.selectedEdgeIds = [];
          });
        },

        renameContext: (contextId, name) => {
          set((state) => {
            saveToHistory(state);
            if (state.contexts[contextId]) {
              state.contexts[contextId].name = name;
            }
          });
        },

        // Node management
        addNode: (contextId, node, position) => {
          const id = generateId();
          set((state) => {
            saveToHistory(state);
            if (state.contexts[contextId]) {
              state.contexts[contextId].nodes[id] = { node, position };
            }
          });
          return id;
        },

        updateNode: (contextId, nodeId, updates) => {
          set((state) => {
            saveToHistory(state);
            const context = state.contexts[contextId];
            if (context?.nodes[nodeId]) {
              Object.assign(context.nodes[nodeId].node, updates);
            }
          });
        },

        deleteNode: (contextId, nodeId) => {
          set((state) => {
            saveToHistory(state);
            const context = state.contexts[contextId];
            if (context) {
              delete context.nodes[nodeId];
              // Remove morphisms connected to this node
              context.morphisms = context.morphisms.filter(
                (m) => m.sourceId !== nodeId && m.targetId !== nodeId
              );
              // Remove from selection
              state.selectedNodeIds = state.selectedNodeIds.filter((id) => id !== nodeId);
            }
          });
        },

        moveNode: (contextId, nodeId, position) => {
          set((state) => {
            const context = state.contexts[contextId];
            if (context?.nodes[nodeId]) {
              context.nodes[nodeId].position = position;
            }
          });
        },

        // Field management
        addField: (contextId, nodeId, field) => {
          set((state) => {
            saveToHistory(state);
            const nodeData = state.contexts[contextId]?.nodes[nodeId];
            if (nodeData && (nodeData.node.kind === 'entity' || nodeData.node.kind === 'value')) {
              nodeData.node.fields.push({ ...field, id: generateId() });
            }
          });
        },

        updateField: (contextId, nodeId, fieldId, updates) => {
          set((state) => {
            saveToHistory(state);
            const nodeData = state.contexts[contextId]?.nodes[nodeId];
            if (nodeData && (nodeData.node.kind === 'entity' || nodeData.node.kind === 'value')) {
              const field = nodeData.node.fields.find((f) => f.id === fieldId);
              if (field) {
                Object.assign(field, updates);
              }
            }
          });
        },

        deleteField: (contextId, nodeId, fieldId) => {
          set((state) => {
            saveToHistory(state);
            const nodeData = state.contexts[contextId]?.nodes[nodeId];
            if (nodeData && (nodeData.node.kind === 'entity' || nodeData.node.kind === 'value')) {
              nodeData.node.fields = nodeData.node.fields.filter((f) => f.id !== fieldId);
            }
          });
        },

        // Enum variant management
        addVariant: (contextId, nodeId, variant) => {
          set((state) => {
            saveToHistory(state);
            const nodeData = state.contexts[contextId]?.nodes[nodeId];
            if (nodeData && nodeData.node.kind === 'enum') {
              nodeData.node.variants.push({ ...variant, id: generateId() });
            }
          });
        },

        updateVariant: (contextId, nodeId, variantId, updates) => {
          set((state) => {
            saveToHistory(state);
            const nodeData = state.contexts[contextId]?.nodes[nodeId];
            if (nodeData && nodeData.node.kind === 'enum') {
              const variant = nodeData.node.variants.find((v) => v.id === variantId);
              if (variant) {
                Object.assign(variant, updates);
              }
            }
          });
        },

        deleteVariant: (contextId, nodeId, variantId) => {
          set((state) => {
            saveToHistory(state);
            const nodeData = state.contexts[contextId]?.nodes[nodeId];
            if (nodeData && nodeData.node.kind === 'enum') {
              nodeData.node.variants = nodeData.node.variants.filter((v) => v.id !== variantId);
            }
          });
        },

        // Morphism management
        addMorphism: (contextId, morphism) => {
          const id = generateId();
          set((state) => {
            saveToHistory(state);
            if (state.contexts[contextId]) {
              state.contexts[contextId].morphisms.push({ ...morphism, id });
            }
          });
          return id;
        },

        updateMorphism: (contextId, morphismId, updates) => {
          set((state) => {
            saveToHistory(state);
            const context = state.contexts[contextId];
            if (context) {
              const morphism = context.morphisms.find((m) => m.id === morphismId);
              if (morphism) {
                Object.assign(morphism, updates);
              }
            }
          });
        },

        deleteMorphism: (contextId, morphismId) => {
          set((state) => {
            saveToHistory(state);
            const context = state.contexts[contextId];
            if (context) {
              context.morphisms = context.morphisms.filter((m) => m.id !== morphismId);
              state.selectedEdgeIds = state.selectedEdgeIds.filter((id) => id !== morphismId);
            }
          });
        },

        // Context map management
        addContextMap: (map) => {
          const id = generateId();
          set((state) => {
            saveToHistory(state);
            state.contextMaps.push({ ...map, id });
          });
          return id;
        },

        updateContextMap: (mapId, updates) => {
          set((state) => {
            saveToHistory(state);
            const map = state.contextMaps.find((m) => m.id === mapId);
            if (map) {
              Object.assign(map, updates);
            }
          });
        },

        deleteContextMap: (mapId) => {
          set((state) => {
            saveToHistory(state);
            state.contextMaps = state.contextMaps.filter((m) => m.id !== mapId);
          });
        },

        // Selection
        setSelectedNodes: (nodeIds) => {
          set((state) => {
            state.selectedNodeIds = nodeIds;
          });
        },

        setSelectedEdges: (edgeIds) => {
          set((state) => {
            state.selectedEdgeIds = edgeIds;
          });
        },

        clearSelection: () => {
          set((state) => {
            state.selectedNodeIds = [];
            state.selectedEdgeIds = [];
          });
        },

        // Validation
        setValidationResult: (result) => {
          set((state) => {
            state.validationResult = result;
          });
        },

        // UI state
        togglePalette: () => {
          set((state) => {
            state.isPaletteOpen = !state.isPaletteOpen;
          });
        },

        togglePropertiesPanel: () => {
          set((state) => {
            state.isPropertiesPanelOpen = !state.isPropertiesPanelOpen;
          });
        },

        // History
        undo: () => {
          if (history.past.length === 0) return;
          const current = get();
          const previous = history.past.pop()!;
          history.future.push(JSON.parse(JSON.stringify(current)));
          set(previous);
        },

        redo: () => {
          if (history.future.length === 0) return;
          const current = get();
          const next = history.future.pop()!;
          history.past.push(JSON.parse(JSON.stringify(current)));
          set(next);
        },

        canUndo: () => history.past.length > 0,
        canRedo: () => history.future.length > 0,

        // Import/Export
        exportModel: () => {
          const state = get();
          // TODO: Convert to .sddd format using WASM
          return JSON.stringify({ contexts: state.contexts, contextMaps: state.contextMaps }, null, 2);
        },

        importModel: (content) => {
          try {
            // Try to parse as JSON first (for .sddd.json files)
            const data = JSON.parse(content);
            set((state) => {
              saveToHistory(state);
              if (data.contexts) {
                state.contexts = data.contexts;
              }
              if (data.contextMaps) {
                state.contextMaps = data.contextMaps;
              }
              // Set first context as active if available
              const contextIds = Object.keys(state.contexts);
              state.activeContextId = contextIds.length > 0 ? contextIds[0] : null;
              state.selectedNodeIds = [];
              state.selectedEdgeIds = [];
            });
          } catch {
            // Not JSON, try parsing as .sddd text format using WASM
            if (!isWasmInitialized()) {
              throw new Error('WASM module not initialized. Please wait for the app to fully load.');
            }

            const result: ParseResult = wasmParse(content);
            if (!result.success || !result.data) {
              throw new Error(result.error || 'Failed to parse .sddd file');
            }

            // Convert parsed data to our internal format
            const contexts: Record<string, ContextData> = {};
            let yOffset = 0;

            for (const ctx of result.data.contexts) {
              const contextId = generateId();
              const nodes: Record<string, { node: DomainNode; position: { x: number; y: number } }> = {};
              const morphisms: Morphism[] = [];
              const nodeIdMap: Record<string, string> = {};

              let x = 50;
              let y = 50;

              // Process entities
              for (const entity of ctx.entities || []) {
                const nodeId = generateId();
                nodeIdMap[entity.name] = nodeId;
                nodes[nodeId] = {
                  node: {
                    kind: 'entity',
                    name: entity.name,
                    fields: (entity.fields || []).map((f) => ({
                      id: generateId(),
                      name: f.name,
                      type: f.type_name,
                      optional: f.optional || false,
                    })),
                  },
                  position: { x, y },
                };
                x += 250;
                if (x > 800) {
                  x = 50;
                  y += 200;
                }
              }

              // Process value objects
              for (const value of ctx.value_objects || []) {
                const nodeId = generateId();
                nodeIdMap[value.name] = nodeId;
                nodes[nodeId] = {
                  node: {
                    kind: 'value',
                    name: value.name,
                    fields: (value.fields || []).map((f) => ({
                      id: generateId(),
                      name: f.name,
                      type: f.type_name,
                      optional: f.optional || false,
                    })),
                  },
                  position: { x, y },
                };
                x += 250;
                if (x > 800) {
                  x = 50;
                  y += 200;
                }
              }

              // Process enums
              for (const enumType of ctx.enums || []) {
                const nodeId = generateId();
                nodeIdMap[enumType.name] = nodeId;
                nodes[nodeId] = {
                  node: {
                    kind: 'enum',
                    name: enumType.name,
                    variants: (enumType.variants || []).map((v) => ({
                      id: generateId(),
                      name: v.name,
                      payload: v.has_payload ? 'unknown' : undefined,
                    })),
                  },
                  position: { x, y },
                };
                x += 250;
                if (x > 800) {
                  x = 50;
                  y += 200;
                }
              }

              // Process aggregates
              for (const agg of ctx.aggregates || []) {
                const nodeId = generateId();
                nodeIdMap[agg.name] = nodeId;
                nodes[nodeId] = {
                  node: {
                    kind: 'aggregate',
                    name: agg.name,
                    rootId: agg.root ? nodeIdMap[agg.root] || '' : '',
                    memberIds: (agg.contains || []).map((m: string) => nodeIdMap[m] || '').filter(Boolean),
                    invariants: [],
                  },
                  position: { x, y },
                };
                x += 250;
                if (x > 800) {
                  x = 50;
                  y += 200;
                }
              }

              // Process morphisms
              for (const morph of ctx.morphisms || []) {
                const sourceId = nodeIdMap[morph.source];
                const targetId = nodeIdMap[morph.target];
                if (sourceId && targetId) {
                  morphisms.push({
                    id: generateId(),
                    sourceId,
                    targetId,
                    name: morph.name || 'relates to',
                    cardinality: 'one',
                  });
                }
              }

              contexts[contextId] = {
                id: contextId,
                name: ctx.name,
                nodes,
                morphisms,
              };

              yOffset += y + 200;
            }

            set((state) => {
              saveToHistory(state);
              state.contexts = contexts;
              state.contextMaps = [];
              const contextIds = Object.keys(contexts);
              state.activeContextId = contextIds.length > 0 ? contextIds[0] : null;
              state.selectedNodeIds = [];
              state.selectedEdgeIds = [];
            });
          }
        },

        reset: () => {
          set(initialState);
          history.past = [];
          history.future = [];
        },
      })),
      {
        name: 'sketchddd-domain-store',
        partialize: (state) => ({
          contexts: state.contexts,
          contextMaps: state.contextMaps,
          activeContextId: state.activeContextId,
        }),
      }
    ),
    { name: 'DomainStore' }
  )
);
