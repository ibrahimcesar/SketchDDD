import { useCallback, useMemo } from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  useNodesState,
  useEdgesState,
  addEdge,
  BackgroundVariant,
  MarkerType,
} from '@xyflow/react';
import type { Connection, NodeTypes, EdgeTypes } from '@xyflow/react';
import '@xyflow/react/dist/style.css';

import { useDomainStore } from '@/stores';
import { ContextNode } from './ContextNode';
import { ContextMapEdge } from './ContextMapEdge';
import type { ContextMapPattern, ContextMap } from '@/types';

const nodeTypes: NodeTypes = {
  boundedContext: ContextNode,
};

const edgeTypes: EdgeTypes = {
  contextMap: ContextMapEdge,
};

interface ContextMapCanvasProps {
  onContextClick: (contextId: string) => void;
  onMapEdit: (map: ContextMap) => void;
}

export function ContextMapCanvas({ onContextClick, onMapEdit }: ContextMapCanvasProps) {
  const { contexts, contextMaps, addContextMap } = useDomainStore();

  // Convert contexts to React Flow nodes
  const initialNodes = useMemo(() => {
    const contextList = Object.entries(contexts);
    const cols = Math.ceil(Math.sqrt(contextList.length));

    return contextList.map(([id, context], index) => ({
      id,
      type: 'boundedContext',
      position: {
        x: (index % cols) * 300 + 50,
        y: Math.floor(index / cols) * 250 + 50,
      },
      data: {
        name: context.name,
        nodeCount: Object.keys(context.nodes).length,
        morphismCount: context.morphisms.length,
        onDoubleClick: () => onContextClick(id),
      },
    }));
  }, [contexts, onContextClick]);

  // Convert context maps to React Flow edges
  const initialEdges = useMemo(() => {
    return contextMaps.map((map) => ({
      id: map.id,
      type: 'contextMap',
      source: map.sourceContextId,
      target: map.targetContextId,
      label: map.name,
      markerEnd: {
        type: MarkerType.ArrowClosed,
      },
      data: {
        pattern: map.pattern,
        mappingCount: map.mappings.length,
        onEdit: () => onMapEdit(map),
      },
    }));
  }, [contextMaps, onMapEdit]);

  const [nodes, , onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback(
    (params: Connection) => {
      if (!params.source || !params.target) return;
      if (params.source === params.target) return;

      // Check if connection already exists
      const existingMap = contextMaps.find(
        (m) =>
          (m.sourceContextId === params.source && m.targetContextId === params.target) ||
          (m.sourceContextId === params.target && m.targetContextId === params.source)
      );

      if (existingMap) {
        alert('A relationship already exists between these contexts');
        return;
      }

      const sourceContext = contexts[params.source];
      const targetContext = contexts[params.target];
      const mapName = `${sourceContext.name} → ${targetContext.name}`;

      const mapId = addContextMap({
        name: mapName,
        sourceContextId: params.source,
        targetContextId: params.target,
        pattern: 'CustomerSupplier',
        mappings: [],
      });

      setEdges((eds) =>
        addEdge(
          {
            ...params,
            id: mapId,
            type: 'contextMap',
            data: {
              pattern: 'CustomerSupplier' as ContextMapPattern,
              mappingCount: 0,
              onEdit: () => {
                const map = useDomainStore.getState().contextMaps.find((m) => m.id === mapId);
                if (map) onMapEdit(map);
              },
            },
          },
          eds
        )
      );
    },
    [contexts, contextMaps, addContextMap, setEdges, onMapEdit]
  );

  if (Object.keys(contexts).length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center bg-slate-100 dark:bg-slate-900">
        <div className="text-center text-slate-500 dark:text-slate-400">
          <p className="text-lg mb-2">No bounded contexts</p>
          <p className="text-sm">Create some bounded contexts to build a context map</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex-1 h-full">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        fitView
        defaultEdgeOptions={{
          type: 'contextMap',
        }}
      >
        <Background variant={BackgroundVariant.Dots} gap={24} size={1} />
        <Controls />
        <MiniMap
          nodeStrokeWidth={3}
          zoomable
          pannable
          className="bg-white dark:bg-slate-800"
        />
      </ReactFlow>
    </div>
  );
}
