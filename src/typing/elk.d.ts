/**
 * Type definitions for ELK (Eclipse Layout Kernel) usage in the AuralStudio project.
 *
 * These interfaces describe the structure of graphs, nodes, and edges as used by ELK's layout engine.
 * Use these types to ensure type safety when constructing graphs for layout and when handling layout results.
 *
 * Usage:
 *   import type { ELKGraph, ELKNode, ELKEdge, ELKLayoutResult, ELKInstance } from './typing/elk';
 *
 * See: https://www.eclipse.org/elk/
 */

/**
 * Represents a node in an ELK graph. Nodes can have children (for compound graphs),
 * and can store layout-relevant properties such as width, height, and position.
 */
export interface ELKNode {
  /** Unique identifier for the node */
  id: string;
  /** Optional type string for custom node types */
  type?: string;
  /** X position after layout (optional, set by ELK) */
  x?: number;
  /** Y position after layout (optional, set by ELK) */
  y?: number;
  /** Width of the node (used for layout calculation) */
  width?: number;
  /** Height of the node (used for layout calculation) */
  height?: number;
  /** Arbitrary data payload for the node (not used by ELK) */
  data?: any;
  /** Optional style object for custom rendering */
  style?: any;
  /** Child nodes (for compound graphs) */
  children?: ELKNode[];
}

/**
 * Represents an edge in an ELK graph. Edges connect nodes by their ids.
 * The 'sources' and 'targets' arrays are required by ELK, while 'source' and 'target'
 * are used for compatibility with other graph libraries.
 */
export interface ELKEdge {
  /** Unique identifier for the edge */
  id: string;
  /** Array of source node ids (usually length 1) */
  sources: string[];
  /** Array of target node ids (usually length 1) */
  targets: string[];
  /** Optional single source node id (for compatibility) */
  source?: string;
  /** Optional single target node id (for compatibility) */
  target?: string;
  /** Optional type string for custom edge types */
  type?: string;
  /** Optional style object for custom rendering */
  style?: any;
}

/**
 * Represents the root graph object passed to ELK for layout.
 * Contains all nodes (children) and edges, as well as layout options.
 */
export interface ELKGraph {
  /** Unique identifier for the root graph */
  id: string;
  /** Layout options for ELK (see ELK documentation for available options) */
  layoutOptions?: Record<string, any>;
  /** Top-level nodes in the graph */
  children?: ELKNode[];
  /** Edges connecting nodes in the graph */
  edges?: ELKEdge[];
}

/**
 * The result of an ELK layout operation. Contains positioned nodes and edges.
 */
export interface ELKLayoutResult {
  /** Nodes with computed positions and sizes */
  children?: ELKNode[];
  /** Edges with computed routing (if any) */
  edges?: ELKEdge[];
}

/**
 * Interface for an ELK layout engine instance. Use the 'layout' method to compute a layout for a graph.
 */
export interface ELKInstance {
  /**
   * Computes a layout for the given graph and returns a promise with the result.
   * @param graph The graph to layout
   * @returns A promise resolving to the layout result
   */
  layout(graph: ELKGraph): Promise<ELKLayoutResult>;
} 