/**
 * D2: Layout in Web Worker — runs ELK layout off the main thread.
 * Receives nodes/edges/options, returns positions.
 */

import type { ElkExtendedEdge, ElkNode } from 'elkjs';

import ElkConstructor from 'elkjs/lib/elk.bundled.js';

const elk = new ElkConstructor();

const DIRECTION_MAP: Record<string, string> = {
  BT: 'UP',
  LR: 'RIGHT',
  RL: 'LEFT',
  TB: 'DOWN',
};

export interface ElkOptionsPayload {
  direction: 'TB' | 'LR' | 'BT' | 'RL';
  nodeWidth: number;
  nodeHeight: number;
  rankSep: number;
  nodeSep: number;
  marginX: number;
  marginY: number;
}

export interface LayoutRequest {
  type: 'layout';
  nodes: { id: string }[];
  edges: { id: string; source: string; target: string }[];
  options: ElkOptionsPayload;
}

export interface LayoutResponse {
  type: 'result';
  positions: { id: string; x: number; y: number }[];
}

const runElkLayout = (
  nodes: { id: string }[],
  edges: { id: string; source: string; target: string }[],
  options: ElkOptionsPayload,
): Promise<{ id: string; x: number; y: number }[]> => {
  if (nodes.length === 0) {
    return Promise.resolve([]);
  }

  const graph: ElkNode = {
    children: nodes.map((n) => ({
      height: options.nodeHeight,
      id: n.id,
      width: options.nodeWidth,
    })),
    edges: edges.map((e) => ({
      id: e.id,
      sources: [e.source],
      targets: [e.target],
    })) as ElkExtendedEdge[],
    id: 'root',
    layoutOptions: {
      'elk.algorithm': 'layered',
      'elk.direction': DIRECTION_MAP[options.direction] ?? 'DOWN',
      'elk.layered.spacing.nodeNodeBetweenLayers': String(options.rankSep),
      'elk.padding': `[left=${options.marginX}, top=${options.marginY}, right=${options.marginX}, bottom=${options.marginY}]`,
      'elk.spacing.nodeNode': String(options.nodeSep),
    },
  };

  return elk.layout(graph).then((result) => {
    const positions: { id: string; x: number; y: number }[] = [];
    for (const child of result.children ?? []) {
        if (child.x !== undefined && child.y !== undefined) {
          positions.push({ id: child.id, x: child.x, y: child.y });
        }
    }
    return positions;
  });
};

const getErrorMessage = (error: unknown): string => {
  if (error instanceof Error) {
    return error.message;
  }
  return String(error);
};

globalThis.addEventListener('message', (ev: MessageEvent<LayoutRequest>): void => {
  const msg = ev.data;
  if (msg.type !== 'layout') {
    return;
  }

  runElkLayout(msg.nodes, msg.edges, msg.options)
    .then((positions) => {
      self.postMessage(
        {
          positions,
          type: 'result',
        } satisfies LayoutResponse,
        self.location.origin,
      );
      return undefined;
    })
    .catch((error: unknown) => {
      self.postMessage(
        {
          error: getErrorMessage(error),
          type: 'error',
        },
        self.location.origin,
      );
      return undefined;
    });
});
