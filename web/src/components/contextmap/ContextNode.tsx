import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';
import type { NodeProps } from '@xyflow/react';
import { Globe, Box, ArrowRight } from 'lucide-react';

interface ContextNodeData {
  name: string;
  nodeCount: number;
  morphismCount: number;
  onDoubleClick: () => void;
}

export const ContextNode = memo(function ContextNode({ data, selected }: NodeProps) {
  const contextData = data as unknown as ContextNodeData;

  return (
    <div
      onDoubleClick={contextData.onDoubleClick}
      className={`
        min-w-[200px] rounded-xl border-2 bg-white dark:bg-slate-800 shadow-lg cursor-pointer
        ${selected ? 'border-primary ring-2 ring-primary/20' : 'border-slate-300 dark:border-slate-600'}
        hover:border-primary/70 transition-all
      `}
    >
      {/* Header */}
      <div className="flex items-center gap-2 px-4 py-3 bg-gradient-to-r from-violet-500 to-purple-500 text-white rounded-t-lg">
        <Globe className="w-5 h-5" />
        <span className="font-bold">{contextData.name}</span>
      </div>

      {/* Content */}
      <div className="p-4">
        <div className="flex items-center justify-between text-sm text-slate-600 dark:text-slate-400">
          <div className="flex items-center gap-1">
            <Box className="w-4 h-4" />
            <span>{contextData.nodeCount} concepts</span>
          </div>
          <div className="flex items-center gap-1">
            <ArrowRight className="w-4 h-4" />
            <span>{contextData.morphismCount} relations</span>
          </div>
        </div>
        <div className="mt-2 text-xs text-slate-500 dark:text-slate-500 text-center">
          Double-click to view details
        </div>
      </div>

      {/* Handles for connections */}
      <Handle
        type="target"
        position={Position.Left}
        className="w-3 h-3 bg-purple-500 border-2 border-white"
      />
      <Handle
        type="source"
        position={Position.Right}
        className="w-3 h-3 bg-purple-500 border-2 border-white"
      />
      <Handle
        type="target"
        position={Position.Top}
        id="top-target"
        className="w-3 h-3 bg-purple-500 border-2 border-white"
      />
      <Handle
        type="source"
        position={Position.Bottom}
        id="bottom-source"
        className="w-3 h-3 bg-purple-500 border-2 border-white"
      />
    </div>
  );
});
