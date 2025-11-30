# ADR-0011: State Management Strategy

## Status
Accepted

## Context
The SketchDDD visual builder is a complex React application that needs to manage:

1. **Domain Model State** - The current model being edited
2. **UI State** - Selected items, panel visibility, zoom level
3. **History State** - Undo/redo stack
4. **Validation State** - Real-time validation results
5. **Persistence State** - Sync status, autosave

We need a state management approach that:
- Handles complex nested state (aggregates containing entities)
- Supports undo/redo efficiently
- Enables real-time validation without blocking UI
- Integrates with React Flow for canvas state
- Persists to localStorage/backend

Options considered:
- **Redux** - Mature, but verbose and complex for this use case
- **Zustand** - Lightweight, simple API, good React integration
- **Jotai** - Atomic state, good for derived values
- **MobX** - Observable state, automatic tracking
- **React Context** - Built-in, but doesn't scale well

## Decision

### Primary: Zustand for Application State

Zustand provides the right balance of simplicity and power:

```typescript
// stores/model.ts
import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import { persist } from 'zustand/middleware';

interface ModelState {
  // Model data
  contexts: BoundedContext[];
  contextMaps: ContextMap[];

  // Actions
  addContext: (name: string) => void;
  removeContext: (id: string) => void;
  addEntity: (contextId: string, entity: Entity) => void;
  updateEntity: (contextId: string, entityId: string, updates: Partial<Entity>) => void;
  // ... more actions
}

export const useModelStore = create<ModelState>()(
  persist(
    immer((set, get) => ({
      contexts: [],
      contextMaps: [],

      addContext: (name) => set((state) => {
        state.contexts.push({
          id: generateId(),
          name,
          entities: [],
          valueObjects: [],
          aggregates: [],
          morphisms: [],
        });
      }),

      removeContext: (id) => set((state) => {
        state.contexts = state.contexts.filter(c => c.id !== id);
      }),

      addEntity: (contextId, entity) => set((state) => {
        const context = state.contexts.find(c => c.id === contextId);
        if (context) {
          context.entities.push(entity);
        }
      }),
      // ...
    })),
    { name: 'sketchddd-model' }
  )
);
```

### Store Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Stores                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │ ModelStore  │  │  UIStore    │  │    HistoryStore     │  │
│  │             │  │             │  │                     │  │
│  │ - contexts  │  │ - selection │  │ - past: State[]     │  │
│  │ - maps      │  │ - panels    │  │ - future: State[]   │  │
│  │ - entities  │  │ - zoom      │  │ - undo()            │  │
│  │ - morphisms │  │ - tool      │  │ - redo()            │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
│         │                │                    │             │
│         └────────────────┼────────────────────┘             │
│                          ↓                                  │
│              ┌─────────────────────┐                        │
│              │  ValidationStore    │                        │
│              │  (derived/computed) │                        │
│              └─────────────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

### UI State Store

```typescript
// stores/ui.ts
interface UIState {
  // Selection
  selectedIds: string[];
  hoveredId: string | null;

  // Panels
  leftPanelOpen: boolean;
  rightPanelOpen: boolean;
  bottomPanelOpen: boolean;

  // Canvas
  zoom: number;
  pan: { x: number; y: number };

  // Tool
  activeTool: 'select' | 'pan' | 'addEntity' | 'addMorphism';

  // Actions
  select: (ids: string[]) => void;
  toggleSelection: (id: string) => void;
  clearSelection: () => void;
  setZoom: (zoom: number) => void;
  // ...
}

export const useUIStore = create<UIState>()((set) => ({
  selectedIds: [],
  hoveredId: null,
  leftPanelOpen: true,
  rightPanelOpen: true,
  bottomPanelOpen: false,
  zoom: 1,
  pan: { x: 0, y: 0 },
  activeTool: 'select',

  select: (ids) => set({ selectedIds: ids }),
  toggleSelection: (id) => set((state) => ({
    selectedIds: state.selectedIds.includes(id)
      ? state.selectedIds.filter(i => i !== id)
      : [...state.selectedIds, id]
  })),
  clearSelection: () => set({ selectedIds: [] }),
  setZoom: (zoom) => set({ zoom: Math.max(0.1, Math.min(2, zoom)) }),
}));
```

### History/Undo-Redo Store

```typescript
// stores/history.ts
import { temporal } from 'zundo';

// Wrap model store with temporal middleware
export const useModelStore = create<ModelState>()(
  temporal(
    persist(
      immer((set, get) => ({
        // ... model state and actions
      })),
      { name: 'sketchddd-model' }
    ),
    {
      limit: 100, // Keep last 100 states
      equality: (a, b) => JSON.stringify(a) === JSON.stringify(b),
    }
  )
);

// Access undo/redo
const { undo, redo, pastStates, futureStates } = useModelStore.temporal.getState();
```

### Validation Store (Derived)

```typescript
// stores/validation.ts
import { useModelStore } from './model';
import { validate } from 'sketchddd-wasm';

// Computed validation using useSyncExternalStore pattern
export function useValidation() {
  const model = useModelStore((state) => ({
    contexts: state.contexts,
    contextMaps: state.contextMaps,
  }));

  // Debounced validation
  const [result, setResult] = useState<ValidationResult | null>(null);

  useEffect(() => {
    const timeout = setTimeout(() => {
      const validationResult = validate(model);
      setResult(validationResult);
    }, 300); // Debounce 300ms

    return () => clearTimeout(timeout);
  }, [model]);

  return result;
}
```

### React Flow Integration

```typescript
// stores/canvas.ts
import { useReactFlow, Node, Edge } from 'reactflow';

// Convert model to React Flow nodes/edges
export function useCanvasState() {
  const contexts = useModelStore((state) => state.contexts);

  const nodes: Node[] = useMemo(() =>
    contexts.flatMap(ctx => [
      // Context as group node
      {
        id: ctx.id,
        type: 'contextNode',
        data: { label: ctx.name },
        position: ctx.position ?? { x: 0, y: 0 },
      },
      // Entities as nodes inside context
      ...ctx.entities.map(entity => ({
        id: entity.id,
        type: 'entityNode',
        data: { entity },
        parentNode: ctx.id,
        position: entity.position ?? { x: 0, y: 0 },
      })),
    ]), [contexts]);

  const edges: Edge[] = useMemo(() =>
    contexts.flatMap(ctx =>
      ctx.morphisms.map(m => ({
        id: m.id,
        source: m.sourceId,
        target: m.targetId,
        label: m.name,
      }))
    ), [contexts]);

  return { nodes, edges };
}
```

### Persistence Strategy

```typescript
// Autosave to localStorage
const useAutosave = () => {
  const model = useModelStore();

  useEffect(() => {
    const unsubscribe = useModelStore.subscribe(
      (state) => {
        // Debounced save
        localStorage.setItem('sketchddd-model', JSON.stringify({
          contexts: state.contexts,
          contextMaps: state.contextMaps,
        }));
      }
    );
    return unsubscribe;
  }, []);
};

// Load on startup
const useModelLoader = () => {
  useEffect(() => {
    const saved = localStorage.getItem('sketchddd-model');
    if (saved) {
      const data = JSON.parse(saved);
      useModelStore.setState(data);
    }
  }, []);
};
```

### DevTools Integration

```typescript
// Enable Redux DevTools for debugging
import { devtools } from 'zustand/middleware';

export const useModelStore = create<ModelState>()(
  devtools(
    temporal(
      persist(
        immer((set, get) => ({
          // ...
        })),
        { name: 'sketchddd-model' }
      )
    ),
    { name: 'SketchDDD Model' }
  )
);
```

## Consequences

### Positive
- Simple, minimal boilerplate
- Built-in persistence with `persist` middleware
- Undo/redo with `zundo` temporal middleware
- TypeScript-first design
- Works with React DevTools
- Easy testing (stores are plain functions)
- Immer integration for immutable updates

### Negative
- Less structured than Redux (no actions/reducers)
- Manual subscription management for derived state
- Middleware order matters

### Neutral
- Separate stores for different concerns
- React Flow manages its own internal state
- Validation computed on-demand

## References
- [Issue #14: Scaffold React application](https://github.com/ibrahimcesar/SketchDDD/issues/14)
- [Issue #22: Implement undo/redo functionality](https://github.com/ibrahimcesar/SketchDDD/issues/22)
- [Zustand](https://github.com/pmndrs/zustand)
- [Zundo](https://github.com/charkour/zundo) - Undo/redo middleware
- [Immer](https://immerjs.github.io/immer/) - Immutable state updates
- [React Flow](https://reactflow.dev/) - Canvas library
