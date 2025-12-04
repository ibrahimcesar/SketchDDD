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

        importModel: (sddd) => {
          try {
            const data = JSON.parse(sddd);
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
          } catch (error) {
            console.error('Failed to import model:', error);
            throw new Error('Invalid .sddd file format');
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
