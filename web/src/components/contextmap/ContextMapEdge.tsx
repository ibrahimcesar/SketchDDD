import { memo } from 'react';
import { BaseEdge, EdgeLabelRenderer, getBezierPath } from '@xyflow/react';
import type { EdgeProps } from '@xyflow/react';
import type { ContextMapPattern } from '@/types';

interface ContextMapEdgeData {
  pattern: ContextMapPattern;
  mappingCount: number;
  onEdit: () => void;
}

const patternLabels: Record<ContextMapPattern, string> = {
  SharedKernel: 'SK',
  CustomerSupplier: 'CS',
  Conformist: 'CF',
  AntiCorruptionLayer: 'ACL',
  OpenHostService: 'OHS',
  PublishedLanguage: 'PL',
  Partnership: 'P',
  SeparateWays: 'SW',
};

const patternColors: Record<ContextMapPattern, string> = {
  SharedKernel: 'bg-blue-500',
  CustomerSupplier: 'bg-green-500',
  Conformist: 'bg-yellow-500',
  AntiCorruptionLayer: 'bg-red-500',
  OpenHostService: 'bg-purple-500',
  PublishedLanguage: 'bg-indigo-500',
  Partnership: 'bg-pink-500',
  SeparateWays: 'bg-slate-500',
};

export const ContextMapEdge = memo(function ContextMapEdge({
  id,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourcePosition,
  targetPosition,
  data,
  style = {},
}: EdgeProps) {
  const edgeData = data as unknown as ContextMapEdgeData;

  const [edgePath, labelX, labelY] = getBezierPath({
    sourceX,
    sourceY,
    sourcePosition,
    targetX,
    targetY,
    targetPosition,
  });

  return (
    <>
      <BaseEdge
        id={id}
        path={edgePath}
        style={{
          ...style,
          strokeWidth: 2,
          stroke: '#6366f1',
        }}
      />
      <EdgeLabelRenderer>
        <div
          style={{
            position: 'absolute',
            transform: `translate(-50%, -50%) translate(${labelX}px,${labelY}px)`,
            pointerEvents: 'all',
          }}
          className="nodrag nopan"
        >
          <button
            onClick={edgeData.onEdit}
            className={`
              px-2 py-1 rounded text-xs font-bold text-white shadow-md
              hover:scale-110 transition-transform cursor-pointer
              ${patternColors[edgeData.pattern]}
            `}
            title={`${edgeData.pattern} - Click to edit`}
          >
            {patternLabels[edgeData.pattern]}
            {edgeData.mappingCount > 0 && (
              <span className="ml-1 opacity-75">({edgeData.mappingCount})</span>
            )}
          </button>
        </div>
      </EdgeLabelRenderer>
    </>
  );
});
