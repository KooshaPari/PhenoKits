import type { Edge, Node } from '@xyflow/react';

import { Layers, Zap } from 'lucide-react';
import { memo, useMemo, useState } from 'react';

import type { HybridGraphConfig } from '@/hooks/useHybridGraph';
import type { Item, Link } from '@tracertm/types';

import { Badge } from '@/components/ui/badge';
import { useHybridGraph } from '@/hooks/useHybridGraph';

import { FlowGraphViewInner } from './FlowGraphViewInner';
import { RichNodeDetailPanel } from './sigma/RichNodeDetailPanel';
import { SigmaGraphView } from './SigmaGraphView';

interface HybridGraphViewProps {
  nodes: Node[];
  edges: Edge[];
  onNodeClick?: ((nodeId: string) => void) | undefined;
  onNodeExpand?: ((nodeId: string) => void) | undefined;
  onNodeNavigate?: ((nodeId: string) => void) | undefined;
  config?: HybridGraphConfig | undefined;
  className?: string | undefined;
}

interface GraphDetailNode {
  id: string;
  label: string;
  type: string;
  data: {
    image?: string | undefined;
    progress?: number | undefined;
    status?: string | undefined;
    description?: string | undefined;
    tags?: string[] | undefined;
    [key: string]: unknown;
  };
}

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null;

const isItem = (value: unknown): value is Item => {
  if (!isRecord(value)) {
    return false;
  }
  return (
    typeof value['id'] === 'string' &&
    typeof value['projectId'] === 'string' &&
    typeof value['view'] === 'string' &&
    typeof value['type'] === 'string' &&
    typeof value['title'] === 'string' &&
    typeof value['status'] === 'string' &&
    typeof value['priority'] === 'string' &&
    typeof value['version'] === 'number' &&
    typeof value['createdAt'] === 'string' &&
    typeof value['updatedAt'] === 'string'
  );
};

const isLink = (value: unknown): value is Link => {
  if (!isRecord(value)) {
    return false;
  }
  return (
    typeof value['id'] === 'string' &&
    typeof value['projectId'] === 'string' &&
    typeof value['sourceId'] === 'string' &&
    typeof value['targetId'] === 'string' &&
    typeof value['type'] === 'string' &&
    typeof value['version'] === 'number' &&
    typeof value['createdAt'] === 'string' &&
    typeof value['updatedAt'] === 'string'
  );
};

const getRecordData = (value: unknown): Record<string, unknown> => {
  if (isRecord(value)) {
    return value;
  }
  return {};
};

const toGraphDetailNode = (node: Node): GraphDetailNode => {
  const nodeData = getRecordData(node.data);
  const label = nodeData['label'];

  return {
    data: nodeData,
    id: node.id,
    label: typeof label === 'string' ? label : node.id,
    type: node.type ?? 'default',
  };
};

export const HybridGraphView = memo(function HybridGraphView({
  nodes,
  edges,
  onNodeClick,
  onNodeExpand,
  onNodeNavigate,
  config,
  className = '',
}: HybridGraphViewProps) {
  const { useWebGL, nodeCount, graphologyGraph, setSelectedNodeId } = useHybridGraph(
    nodes,
    edges,
    config,
  );

  const [detailPanelNode, setDetailPanelNode] = useState<GraphDetailNode | null>(null);
  const reactFlowItems = useMemo(() => {
    const timestamp = new Date().toISOString();
    return nodes
      .map((node) => {
        const data = getRecordData(node.data);
        const item = data['item'];
        if (isItem(item)) {
          return item;
        }
        return {
          createdAt: timestamp,
          id: node.id,
          priority: 'medium',
          projectId: 'unknown',
          status: 'todo',
          title: node.id,
          type: node.type ?? 'node',
          updatedAt: timestamp,
          version: 1,
          view: 'feature',
        } satisfies Item;
      })
      .filter((item): item is Item => Boolean(item));
  }, [nodes]);
  const reactFlowLinks = useMemo(
    () =>
      edges
        .map((edge) => {
          const data = getRecordData(edge.data);
          const link = data['link'];
          if (isLink(link)) {
            return link;
          }
          return undefined;
        })
        .filter((link): link is Link => Boolean(link)),
    [edges],
  );

  // Handle node click
  const handleNodeClick = (nodeId: string) => {
    setSelectedNodeId(nodeId);

    // In WebGL mode, open detail panel
    if (useWebGL) {
      const node = nodes.find((n) => n.id === nodeId);
      if (node) {
        setDetailPanelNode(toGraphDetailNode(node));
      }
    }

    // Call parent handler
    if (onNodeClick) {
      onNodeClick(nodeId);
    }
  };

  // Handle node double-click (expand in WebGL mode)
  const handleNodeDoubleClick = (nodeId: string) => {
    if (onNodeExpand) {
      onNodeExpand(nodeId);
    }
  };

  return (
    <div className={`relative h-full w-full ${className}`}>
      {/* Performance mode indicator */}
      <div className='absolute top-4 right-4 z-10 flex items-center gap-2'>
        <Badge variant={useWebGL ? 'default' : 'secondary'} className='text-xs font-medium'>
          {useWebGL ? (
            <>
              <Zap className='mr-1 h-3 w-3' />
              WebGL Mode
            </>
          ) : (
            <>
              <Layers className='mr-1 h-3 w-3' />
              ReactFlow Mode
            </>
          )}
        </Badge>
        <Badge variant='outline' className='text-xs'>
          {nodeCount.toLocaleString()} nodes
        </Badge>
      </div>

      {/* Render graph based on mode */}
      {useWebGL && graphologyGraph ? (
        // WebGL mode (>10k nodes)
        <SigmaGraphView
          graph={graphologyGraph}
          onNodeClick={handleNodeClick}
          onNodeDoubleClick={handleNodeDoubleClick}
          className='h-full w-full'
        />
      ) : (
        // ReactFlow mode (<10k nodes)
        <FlowGraphViewInner
          items={reactFlowItems}
          links={reactFlowLinks}
          onNavigateToItem={handleNodeClick}
        />
      )}

      {/* Rich node detail panel (WebGL mode only) */}
      {useWebGL && detailPanelNode !== null && (
        <RichNodeDetailPanel
          node={detailPanelNode}
          onClose={() => {
            setDetailPanelNode(null);
          }}
          onExpand={onNodeExpand}
          onNavigate={onNodeNavigate}
        />
      )}

      {/* Threshold warning (near threshold) */}
      {!useWebGL && nodeCount > 8000 && (
        <div className='absolute bottom-4 left-4 z-10'>
          <Badge variant='warning' className='text-xs'>
            Approaching 10k node threshold - WebGL mode will activate automatically
          </Badge>
        </div>
      )}
    </div>
  );
});
