/**
 * Type definitions for React Flow types used in the AuralStudio project.
 *
 * This file provides type-safe aliases for the most commonly used React Flow types,
 * making it easier to update or extend them in one place. If you need to add custom
 * node or edge data, extend these types here.
 *
 * Usage:
 *   import type { Node, Edge } from './typing/reactflow';
 *
 * These types are used throughout the digraph rendering and manipulation logic.
 */

import type {
  /**
   * Represents a node in the React Flow graph. Each node has an id, type, position, data, etc.
   * You can extend this type if you want to add custom data fields to your nodes.
   * See: https://reactflow.dev/docs/api/types/#node
   */
  Node as ReactFlowNode,
  /**
   * Represents an edge (connection) between two nodes in React Flow. Each edge has an id, source, target, etc.
   * You can extend this type for custom edge data or styling.
   * See: https://reactflow.dev/docs/api/types/#edge
   */
  Edge as ReactFlowEdge,
  /**
   * A mapping from node type names to React components. Used to render custom node UIs.
   * See: https://reactflow.dev/docs/api/types/#nodetypes
   */
  NodeTypes as ReactFlowNodeTypes,
  /**
   * The React context provider for React Flow. Wrap your flow components with this.
   * See: https://reactflow.dev/docs/api/reactflowprovider/
   */
  ReactFlowProvider as ReactFlowProviderType,
} from 'reactflow';

/**
 * Alias for the React Flow Node type.
 * Extend this if you want to add custom node data.
 */
export type Node = ReactFlowNode;

/**
 * Alias for the React Flow Edge type.
 * Extend this if you want to add custom edge data.
 */
export type Edge = ReactFlowEdge;

/**
 * Alias for the React Flow NodeTypes mapping.
 * Use this to register custom node components.
 */
export type NodeTypes = ReactFlowNodeTypes;

/**
 * Alias for the React FlowProvider component.
 * Use this to wrap your React Flow graphs.
 */
export type ReactFlowProvider = ReactFlowProviderType;

// Optionally, you can extend or augment types here if you need custom data 