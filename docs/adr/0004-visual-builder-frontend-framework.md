# ADR-0004: Visual Builder Frontend Framework

> **Discussion**: [GitHub Discussions #31](https://github.com/ibrahimcesar/SketchDDD/discussions/31)

## Status
Proposed

## Context
The Visual Builder needs to be:
1. A rich, interactive web application
2. Capable of node-based visual editing (diagrams with nodes and edges)
3. Responsive and performant with potentially hundreds of nodes
4. Compatible with our Rust/WASM core engine
5. Maintainable for a small team

We need to choose a frontend framework and supporting libraries. This decision affects development velocity, performance, maintainability, and contributor accessibility.

## Options Under Consideration

### Option A: React + React Flow

**Stack**: React 18+, TypeScript, React Flow, Tailwind CSS, Zustand

| Aspect | Assessment |
|--------|------------|
| **Node-based editing** | React Flow is purpose-built for this; mature, well-documented |
| **Ecosystem** | Largest ecosystem; most npm packages available |
| **WASM integration** | Well-documented patterns; many examples |
| **Performance** | Virtual DOM overhead; React Flow handles canvas optimization |
| **Bundle size** | ~45KB (React) + ~80KB (React Flow) |
| **Learning curve** | Moderate; many developers already know React |
| **Hiring/Contributors** | Easiest to find contributors |

**Pros**:
- React Flow solves the hardest problem (node/edge canvas) out of the box
- Vast ecosystem of UI components (shadcn/ui, Radix, etc.)
- Excellent TypeScript support
- Huge community for troubleshooting

**Cons**:
- Larger bundle size than alternatives
- React's complexity (hooks, effects, re-renders) adds cognitive load
- React Flow creates lock-in for the canvas component

---

### Option B: Vue 3 + Vue Flow

**Stack**: Vue 3, TypeScript, Vue Flow, Tailwind CSS, Pinia

| Aspect | Assessment |
|--------|------------|
| **Node-based editing** | Vue Flow is a Vue port of React Flow; slightly less mature |
| **Ecosystem** | Large ecosystem; fewer options than React but sufficient |
| **WASM integration** | Good support; fewer examples than React |
| **Performance** | Compiler optimizations; generally faster than React |
| **Bundle size** | ~33KB (Vue) + ~75KB (Vue Flow) |
| **Learning curve** | Gentler than React; template syntax is approachable |
| **Hiring/Contributors** | Good pool; smaller than React |

**Pros**:
- Vue Flow provides same capabilities as React Flow
- Vue's reactivity is more intuitive than React hooks
- Smaller bundle than React
- Single-file components are great for organization

**Cons**:
- Vue Flow is less battle-tested than React Flow
- Smaller component ecosystem
- Fewer WASM + Vue examples available

---

### Option C: Svelte 5 + Svelvet

**Stack**: Svelte 5, TypeScript, Svelvet, Tailwind CSS

| Aspect | Assessment |
|--------|------------|
| **Node-based editing** | Svelvet is newer, less feature-complete than React/Vue Flow |
| **Ecosystem** | Smaller but growing; may lack some components |
| **WASM integration** | Straightforward; fewer examples |
| **Performance** | Best performance; compiles to vanilla JS |
| **Bundle size** | ~5KB (Svelte) + ~50KB (Svelvet) - smallest |
| **Learning curve** | Easiest to learn; closest to vanilla HTML/JS |
| **Hiring/Contributors** | Smaller pool; growing popularity |

**Pros**:
- Best performance and smallest bundle
- Simplest mental model; easiest to learn
- No virtual DOM; direct DOM manipulation
- Runes (Svelte 5) provide excellent reactivity

**Cons**:
- Svelvet is least mature of the three canvas libraries
- Smallest ecosystem; may need to build more from scratch
- Fewer developers familiar with Svelte

---

### Option D: Solid.js + Custom Canvas (D3/Canvas API)

**Stack**: Solid.js, TypeScript, D3.js or Canvas API, Tailwind CSS

| Aspect | Assessment |
|--------|------------|
| **Node-based editing** | No ready-made solution; would need custom implementation |
| **Ecosystem** | Small but focused; growing |
| **WASM integration** | Excellent; reactive primitives work well with external state |
| **Performance** | Excellent; fine-grained reactivity without VDOM |
| **Bundle size** | ~7KB (Solid) + custom canvas code |
| **Learning curve** | Moderate; similar to React but different |
| **Hiring/Contributors** | Smallest pool |

**Pros**:
- Maximum performance and control
- No abstraction over the canvas; full flexibility
- Fine-grained reactivity excellent for WASM integration

**Cons**:
- Must build node-based editor from scratch (significant effort)
- Smallest ecosystem and contributor pool
- Higher risk; more custom code to maintain

---

## Decision Matrix

| Criteria (Weight) | React Flow | Vue Flow | Svelte/Svelvet | Solid/Custom |
|-------------------|------------|----------|----------------|--------------|
| Canvas maturity (25%) | 5 | 4 | 3 | 2 |
| Performance (20%) | 3 | 4 | 5 | 5 |
| Ecosystem (20%) | 5 | 4 | 3 | 2 |
| Bundle size (10%) | 2 | 3 | 5 | 4 |
| WASM integration (15%) | 4 | 4 | 4 | 5 |
| Contributor access (10%) | 5 | 4 | 3 | 2 |
| **Weighted Score** | **4.05** | **3.90** | **3.65** | **3.15** |

## Recommendation

**Pending decision.** Initial analysis suggests **React + React Flow** or **Vue + Vue Flow** are the most pragmatic choices due to mature canvas libraries. The choice between them may come down to team preference and contributor considerations.

Questions to resolve:

1. Do we prioritize ecosystem size (React) or slightly better performance (Vue)?
2. Is Svelte's smaller bundle worth the risk of a less mature canvas library?
3. Do we have contributors with strong preferences?

## Decision
[To be filled after discussion]

## Consequences
[To be filled after decision]

## References
- [React Flow](https://reactflow.dev/)
- [Vue Flow](https://vueflow.dev/)
- [Svelvet](https://svelvet.io/)
- [Solid.js](https://solidjs.com/)
- [D3.js](https://d3js.org/)
