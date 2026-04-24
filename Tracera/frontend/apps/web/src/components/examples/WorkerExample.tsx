/**
 * Web Workers Usage Example
 *
 * Demonstrates how to use the worker hooks in a React component
 */

import { useState } from 'react';

import {
  useDataTransformWorker,
  useExportImportWorker,
  useGraphLayoutWorker,
  useSearchIndexWorker,
  useWorkerSupport,
} from '@/hooks/useWorker';

import type { LayoutResult } from '@/workers/graph-layout.worker';
import type { SearchIndex, SearchResult } from '@/workers/search-index.worker';

function GraphLayoutExample() {
  const { worker, status, createProgressCallback } = useGraphLayoutWorker();
  const [layoutResult, setLayoutResult] = useState<LayoutResult | null>(null);

  const handleComputeLayout = async () => {
    if (!worker) {
      return;
    }

    const nodes = [
      { height: 50, id: 'A', width: 100 },
      { height: 50, id: 'B', width: 100 },
      { height: 50, id: 'C', width: 100 },
    ];

    const edges = [
      { id: 'AB', source: 'A', target: 'B' },
      { id: 'BC', source: 'B', target: 'C' },
    ];

    const onProgress = createProgressCallback();

    try {
      const result = await worker.computeLayout(nodes, edges, {
        direction: 'TB',
        type: 'dagre',
        onProgress,
      });
      setLayoutResult(result);
    } catch {
      // Error handling is managed through status.error
    }
  };

  return (
    <div className='rounded border p-4'>
      <h2 className='mb-4 text-lg font-bold'>Graph Layout Worker</h2>

      <div className='mb-4'>
        <div>Status: {status.isReady ? '✅ Ready' : '⏳ Loading...'}</div>
        {status.error !== null && (
          <div className='text-red-500'>Error: {status.error.message}</div>
        )}
        {status.progress > 0 && (
          <div className='mt-2'>
            <div className='mb-1 text-sm'>Progress: {status.progress.toFixed(0)}%</div>
            <div className='w-full rounded bg-gray-200'>
              <div
                className='h-2 rounded bg-blue-500 transition-all'
                style={{ width: `${status.progress}%` }}
              />
            </div>
          </div>
        )}
      </div>

      <button
        onClick={() => {
          void handleComputeLayout();
        }}
        disabled={!status.isReady}
        className='rounded bg-blue-500 px-4 py-2 text-white disabled:opacity-50'
      >
        Compute Layout
      </button>

      {layoutResult !== null && (
        <div className='mt-4'>
          <h3 className='font-semibold'>Result:</h3>
          <pre className='mt-2 overflow-auto rounded bg-gray-100 p-2 text-xs'>
            {JSON.stringify(layoutResult, null, 2)}
          </pre>
        </div>
      )}
    </div>
  );
}

interface TransformResult {
  aggregated: unknown;
  sorted: { id: number; value: number }[];
  stats: unknown;
}

type SortedPreviewItem = Record<string, unknown> & { id: number; value: number };

const isSortedPreviewItem = (item: Record<string, unknown>): item is SortedPreviewItem =>
  typeof item['id'] === 'number' && typeof item['value'] === 'number';

function DataTransformExample() {
  const { worker, status } = useDataTransformWorker();
  const [result, setResult] = useState<TransformResult | null>(null);

  const handleTransform = async () => {
    if (!worker) {
      return;
    }

    const data = Array.from({ length: 1000 }, (_, i) => ({
      category: ['A', 'B', 'C'][i % 3],
      id: i,
      value: Math.random() * 100,
    }));

    try {
      // Sort data
      const sorted = await worker.sortData(data, {
        direction: 'desc',
        field: 'value',
      });

      // Calculate statistics
      const stats = await worker.calculateStatistics(data, { field: 'value' });

      // Aggregate by category
      const aggregated = await worker.aggregateData(data, {
        aggregateField: 'value',
        aggregationType: 'sum',
        groupByField: 'category',
      });

      setResult({
        aggregated,
        sorted: sorted.slice(0, 10).filter(isSortedPreviewItem),
        stats,
      });
    } catch {
      // Error handling is managed through status.error
    }
  };

  return (
    <div className='rounded border p-4'>
      <h2 className='mb-4 text-lg font-bold'>Data Transform Worker</h2>

      <button
        onClick={() => {
          void handleTransform();
        }}
        disabled={!status.isReady}
        className='rounded bg-green-500 px-4 py-2 text-white disabled:opacity-50'
      >
        Transform 1000 Items
      </button>

      {result !== null && (
        <div className='mt-4 space-y-2'>
          <div>
            <h3 className='font-semibold'>Statistics:</h3>
            <pre className='mt-1 rounded bg-gray-100 p-2 text-xs'>
              {JSON.stringify(result.stats, null, 2)}
            </pre>
          </div>
          <div>
            <h3 className='font-semibold'>Top 10 (sorted):</h3>
            <ul className='mt-1 text-sm'>
              {result.sorted.map((item) => (
                <li key={item.id}>
                  ID: {item.id}, Value: {item.value.toFixed(2)}
                </li>
              ))}
            </ul>
          </div>
          <div>
            <h3 className='font-semibold'>Aggregated by Category:</h3>
            <pre className='mt-1 rounded bg-gray-100 p-2 text-xs'>
              {JSON.stringify(result.aggregated, null, 2)}
            </pre>
          </div>
        </div>
      )}
    </div>
  );
}

function ExportImportExample() {
  const { worker, status } = useExportImportWorker();
  const [exported, setExported] = useState('');

  const handleExport = async () => {
    if (!worker) {
      return;
    }

    const data = [
      { age: 30, id: 1, name: 'Alice' },
      { age: 25, id: 2, name: 'Bob' },
      { age: 35, id: 3, name: 'Charlie' },
    ];

    try {
      const ndjson = await worker.generateNDJSON(data);
      setExported(ndjson);
    } catch {
      // Error handling is managed through status.error
    }
  };

  const handleImport = async () => {
    if (!worker || exported === '') {
      return;
    }

    try {
      const data = await worker.parseNDJSON(exported);

      console.log(`Imported ${data.length} items`);
    } catch {
      // Error handling is managed through status.error
    }
  };

  return (
    <div className='rounded border p-4'>
      <h2 className='mb-4 text-lg font-bold'>Export/Import Worker</h2>

      <div className='mb-4 space-x-2'>
        <button
          onClick={() => {
            void handleExport();
          }}
          disabled={!status.isReady}
          className='rounded bg-purple-500 px-4 py-2 text-white disabled:opacity-50'
        >
          Export to NDJSON
        </button>
        <button
          onClick={() => {
            void handleImport();
          }}
          disabled={!status.isReady || exported === ''}
          className='rounded bg-purple-700 px-4 py-2 text-white disabled:opacity-50'
        >
          Import from NDJSON
        </button>
      </div>

      {exported !== '' && (
        <div className='mt-4'>
          <h3 className='font-semibold'>Exported NDJSON:</h3>
          <pre className='mt-2 overflow-auto rounded bg-gray-100 p-2 text-xs'>{exported}</pre>
        </div>
      )}
    </div>
  );
}

function SearchIndexExample() {
  const { worker, status } = useSearchIndexWorker();
  const [index, setIndex] = useState<SearchIndex | null>(null);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);

  const handleBuildIndex = async () => {
    if (!worker) {
      return;
    }

    const documents = [
      {
        fields: {
          content: 'React is a JavaScript library for building user interfaces',
          title: 'Introduction to React',
        },
        id: '1',
      },
      {
        fields: {
          content: 'Vue is a progressive framework for building user interfaces',
          title: 'Vue.js Basics',
        },
        id: '2',
      },
      {
        fields: {
          content: 'Angular is a platform for building web applications',
          title: 'Angular Guide',
        },
        id: '3',
      },
    ];

    try {
      const newIndex = await worker.buildIndex(documents, {
        title: 2, // Title has 2x weight
        content: 1, // Content has default weight
      });
      setIndex(newIndex);
    } catch {
      // Error handling is managed through status.error
    }
  };

  const handleSearch = async () => {
    if (!worker || index === null || query === '') {
      return;
    }

    try {
      const searchResults = await worker.search(
        index,
        query,
        {
          fuzzy: true,
          maxDistance: 2,
        },
      );
      setResults(searchResults);
    } catch {
      // Error handling is managed through status.error
    }
  };

  return (
    <div className='rounded border p-4'>
      <h2 className='mb-4 text-lg font-bold'>Search Index Worker</h2>

      <div className='space-y-4'>
        <button
          onClick={() => {
            void handleBuildIndex();
          }}
          disabled={!status.isReady}
          className='rounded bg-orange-500 px-4 py-2 text-white disabled:opacity-50'
        >
          Build Index
        </button>

        {index !== null && (
          <>
            <div className='flex gap-2'>
              <input
                type='text'
                value={query}
                onChange={(e) => {
                  setQuery(e.target.value);
                }}
                placeholder='Search query...'
                className='flex-1 rounded border px-3 py-2'
              />
              <button
                onClick={() => {
                  void handleSearch();
                }}
                className='rounded bg-orange-700 px-4 py-2 text-white'
              >
                Search
              </button>
            </div>

            {results.length > 0 && (
              <div className='mt-4'>
                <h3 className='font-semibold'>Results:</h3>
                <ul className='mt-2 space-y-2'>
                  {results.map((result) => (
                    <li key={result.id} className='rounded bg-gray-100 p-2'>
                      <div className='font-medium'>Document {result.id}</div>
                      <div className='text-sm text-gray-600'>Score: {result.score.toFixed(2)}</div>
                    </li>
                  ))}
                </ul>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
}

function WorkerSupportCheck() {
  const { supported, checked } = useWorkerSupport();

  if (!checked) {
    return <div>Checking Web Worker support...</div>;
  }

  return (
    <div className={`rounded border p-4 ${supported ? 'bg-green-50' : 'bg-red-50'}`}>
      <h3 className='mb-2 font-semibold'>Web Worker Support</h3>
      {supported ? (
        <div className='text-green-700'>✅ Web Workers are supported in this browser</div>
      ) : (
        <div className='text-red-700'>
          ❌ Web Workers are not supported. Some features may be unavailable or slower.
        </div>
      )}
    </div>
  );
}

/**
 * Main demo component showing all worker examples
 */
function WebWorkersDemo() {
  return (
    <div className='mx-auto max-w-6xl p-8'>
      <h1 className='mb-8 text-3xl font-bold'>Web Workers Demo</h1>

      <div className='space-y-6'>
        <WorkerSupportCheck />
        <GraphLayoutExample />
        <DataTransformExample />
        <ExportImportExample />
        <SearchIndexExample />
      </div>
    </div>
  );
}

export {
  DataTransformExample,
  ExportImportExample,
  GraphLayoutExample,
  SearchIndexExample,
  WebWorkersDemo,
  WorkerSupportCheck,
};
