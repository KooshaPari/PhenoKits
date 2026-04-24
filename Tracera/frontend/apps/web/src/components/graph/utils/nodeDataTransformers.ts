// Node Data Transformers - Convert Item to type-specific node data
// Implements type-aware node system for the graph view

import type { EpicItem, Item, RequirementItem, TestItem } from '@tracertm/types';

import type { EpicNodeData } from '../nodes/EpicNode';
import type { RequirementNodeData } from '../nodes/RequirementNode';
import type { TestNodeData } from '../nodes/TestNode';
import type { RichNodeData } from '../RichNodePill';

type NormalizedTestStatus = TestNodeData['lastRunStatus'];

const TEST_SAFETY_LEVELS = ['safe', 'quarantined', 'disabled'] as const;
const TEST_EARS_PATTERN_TYPES = [
  'ubiquitous',
  'event_driven',
  'state_driven',
  'optional',
  'unwanted',
] as const;
const TEST_RISK_LEVELS = ['low', 'medium', 'high', 'critical'] as const;
const TEST_VERIFICATION_STATUSES = ['not_verified', 'partially_verified', 'verified'] as const;
const TEST_BUSINESS_VALUES = ['low', 'medium', 'high', 'critical'] as const;

function getMetadataString(metadata: Record<string, unknown>, key: string): string | undefined {
  const value = metadata[key];
  return typeof value === 'string' ? value : undefined;
}

function getMetadataNumber(metadata: Record<string, unknown>, key: string): number | undefined {
  const value = metadata[key];
  return typeof value === 'number' ? value : undefined;
}

function getMetadataEnum<T extends string>(
  metadata: Record<string, unknown>,
  key: string,
  allowed: readonly T[],
): T | undefined {
  const value = metadata[key];
  if (typeof value !== 'string') {
    return undefined;
  }

  for (const candidate of allowed) {
    if (candidate === value) {
      return candidate;
    }
  }

  return undefined;
}

function normalizeTestStatus(status: TestItem['lastExecutionResult']): NormalizedTestStatus {
  if (status === undefined) {
    return undefined;
  }

  if (status === 'blocked') {
    return 'failed';
  }

  if (
    status === 'passed' ||
    status === 'failed' ||
    status === 'skipped' ||
    status === 'error'
  ) {
    return status;
  }

  return undefined;
}

/**
 * Type guards
 */
function isTestItem(item: Item): item is TestItem {
  return item['type'] === 'test' || item['type'] === 'test_case' || item['type'] === 'test_suite';
}

function isRequirementItem(item: Item): item is RequirementItem {
  return item['type'] === 'requirement';
}

function isEpicItem(item: Item): item is EpicItem {
  return item['type'] === 'epic';
}

/**
 * Transform a test item to TestNodeData
 */
function transformTestItem(item: TestItem): Partial<TestNodeData> {
  // Extract test-specific metadata
  const metadata = item['metadata'] ?? {};
  const result: Partial<TestNodeData> = {};

  const coveragePercent = getMetadataNumber(metadata, 'coveragePercent');
  if (coveragePercent !== undefined) result.coveragePercent = coveragePercent;
  const flakinessScore = getMetadataNumber(metadata, 'flakinessScore');
  if (flakinessScore !== undefined) result.flakinessScore = flakinessScore;
  const framework = getMetadataString(metadata, 'framework');
  if (framework !== undefined) result.framework = framework;
  const lastRunStatus = normalizeTestStatus(item['lastExecutionResult']);
  if (lastRunStatus !== undefined) result.lastRunStatus = lastRunStatus;
  const safetyLevel = getMetadataEnum(metadata, 'safetyLevel', TEST_SAFETY_LEVELS);
  if (safetyLevel !== undefined) result.safetyLevel = safetyLevel;
  if (item['testType'] !== undefined) result.testType = item['testType'];

  return result;
}

/**
 * Transform a requirement item to RequirementNodeData
 */
function transformRequirementItem(item: RequirementItem): Partial<RequirementNodeData> {
  const metadata = item['metadata'] ?? {};
  const { qualityMetrics } = item;
  const result: Partial<RequirementNodeData> = {};

  const earsPatternType = getMetadataEnum(metadata, 'earsPatternType', TEST_EARS_PATTERN_TYPES);
  if (earsPatternType !== undefined) result.earsPatternType = earsPatternType;
  const riskLevel = getMetadataEnum(metadata, 'riskLevel', TEST_RISK_LEVELS);
  if (riskLevel !== undefined) result.riskLevel = riskLevel;
  if (qualityMetrics?.completenessScore !== undefined)
    result.verifiabilityScore = qualityMetrics.completenessScore;
  const verificationStatus = getMetadataEnum(metadata, 'verificationStatus', TEST_VERIFICATION_STATUSES);
  if (verificationStatus !== undefined) result.verificationStatus = verificationStatus;
  const wsjfScore = getMetadataNumber(metadata, 'wsjfScore');
  if (wsjfScore !== undefined) result.wsjfScore = wsjfScore;

  return result;
}

/**
 * Transform an epic item to EpicNodeData
 */
function transformEpicItem(item: EpicItem): Partial<EpicNodeData> {
  const metadata = item['metadata'] ?? {};
  const result: Partial<EpicNodeData> = {};

  const businessValue = item['businessValue']
    ? getMetadataEnum(
        { businessValue: item['businessValue'].toLowerCase() },
        'businessValue',
        TEST_BUSINESS_VALUES,
      )
    : getMetadataEnum(metadata, 'businessValue', TEST_BUSINESS_VALUES);
  if (businessValue !== undefined) result.businessValue = businessValue;
  const completedStoryCount = getMetadataNumber(metadata, 'completedStoryCount');
  if (completedStoryCount !== undefined) result.completedStoryCount = completedStoryCount;
  const storyCount = getMetadataNumber(metadata, 'storyCount');
  if (storyCount !== undefined) result.storyCount = storyCount;
  const timelineProgress = getMetadataNumber(metadata, 'timelineProgress');
  if (timelineProgress !== undefined) result.timelineProgress = timelineProgress;

  return result;
}

/**
 * Main transformer: Convert any Item to appropriate node data
 * Returns RichNodeData with type-specific fields merged in
 */
export function itemToNodeData(
  item: Item,
  connections = { incoming: 0, outgoing: 0, total: 0 },
): RichNodeData | TestNodeData | RequirementNodeData | EpicNodeData {
  // Base node data (common to all types)
  const baseData: RichNodeData = {
    connections,
    description: item['description'],
    id: item['id'],
    item,
    label: item['title'],
    status: item['status'],
    type: item['type'],
  };

  // Type-specific transformations
  if (isTestItem(item)) {
    return {
      ...baseData,
      ...transformTestItem(item),
    };
  }

  if (isRequirementItem(item)) {
    return {
      ...baseData,
      ...transformRequirementItem(item),
    };
  }

  if (isEpicItem(item)) {
    return {
      ...baseData,
      ...transformEpicItem(item),
    };
  }

  // Default: return as RichNodeData
  return baseData;
}

/**
 * Batch transform items to node data
 */
export function itemsToNodeData(
  items: Item[],
  connectionMap?: Map<string, { incoming: number; outgoing: number; total: number }>,
): (RichNodeData | TestNodeData | RequirementNodeData | EpicNodeData)[] {
  return items.map((item) => {
    const connections = connectionMap?.get(item['id']) ?? {
      incoming: 0,
      outgoing: 0,
      total: 0,
    };
    return itemToNodeData(item, connections);
  });
}
