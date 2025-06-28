/**
 * Custom node type mapping for React Flow in the AuralStudio project.
 *
 * This constant maps node type names to their corresponding React components.
 * Used to register custom node renderers for the digraph visualization.
 *
 * Usage:
 *   import { nodeTypes } from './typing/nodeTypes';
 *
 * Pass this to the React Flow 'nodeTypes' prop.
 */

import type { NodeTypes } from 'reactflow';
import { RTLNodeComponent, FileNodeComponent } from '../components/Nodes';

/**
 * Mapping of node type names to React components for React Flow.
 * - rtlNode: Renders a regular RTL node
 * - fileNode: Renders the file node
 */
export const nodeTypes: NodeTypes = {
  rtlNode: RTLNodeComponent,
  fileNode: FileNodeComponent,
}; 