/**
 * @fileoverview Renders the acyclic digraph for a given source file. Handles the rendering of
 * individual pieces, nodes, blocks and subtrees.
 */

import { useEffect, useState, useMemo, useCallback } from "react";
import ReactFlow, {
  Node,
  Controls,
  Background,
  NodeTypes,
  ReactFlowProvider,
  Handle,
  Position,
  useUpdateNodeInternals,
} from "reactflow";
import "reactflow/dist/style.css";
import { Canvas, RTLNode, RTLPiece } from "./types";
import { FLEX_COL, FLEX_ROW } from "./styles";
import { RenderNode, FileNodeComponent } from "./components/Nodes";
import function_arrow from "./assets/function_arrow.png";

// The kinds of nodes that would require an arrow to be drawn towards them.
// Currently includes:
//  - 'yes' branch of if-statements
//  - 'no' branch of if-statements
const ARROW_NODES = ["CONDTLY", "CONDTLN"];

// Custom node component for RTL nodes
const RTLNodeComponent = ({ data }: { data: any }) => {
  const { node, selectedAddr, renderedAddr, pieceIx, parentIndents } = data;
  const address = node.address;
  const updateNodeInternals = useUpdateNodeInternals();

  useEffect(() => {
    updateNodeInternals(address);
  }, [address, updateNodeInternals]);

  return (
    <div
      key={address}
      style={{
        background:
          renderedAddr === `${address}.0` && selectedAddr === `${address}.0`
            ? "#f7dc28"
            : "linear-gradient(#FFFFFF2A, #191A1B99)",
        borderRadius: "25px",
        padding: "2px",
      }}
    >
      <div
        id={address.slice(0, address.length - 2)}
        style={{
          height: "100%",
          background:
            "radial-gradient(ellipse 60% 70% at center top, #292B4C, #18191B)",
          borderRadius: "23px",
          padding: "40px",
        }}
      >
        {/* Source handle for outgoing edges */}
        <Handle
          type="source"
          position={Position.Bottom}
          id={`${address}-source`}
          style={{ background: "transparent", color: "transparent" }}
        />

        {/* Target handle for incoming edges */}
        <Handle
          type="target"
          position={Position.Top}
          id={`${address}-target`}
          style={{ background: "white" }}
        />

        {RenderNode(
          node,
          node.address,
          renderedAddr,
          selectedAddr,
          parentIndents,
          pieceIx,
          undefined,
        )}

        <div style={{ ...FLEX_ROW, marginTop: "0.5rem" }}>
          <img
            style={{
              height: "24px",
              marginRight: "5px",
            }}
            src={function_arrow}
          />
          <div style={{ ...FLEX_COL, gap: "0.3rem" }}>
            {node.children.map((child: RTLNode) =>
              RenderNode(
                child,
                child.address,
                renderedAddr,
                selectedAddr,
                parentIndents,
                pieceIx,
                node,
              ),
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

// Custom node types
const nodeTypes: NodeTypes = {
  rtlNode: RTLNodeComponent,
  fileNode: FileNodeComponent,
};

// TODO: When editing, we need to render text boxes for strings + identifiers.
export function DAG(
  payload: Canvas,
  hide: boolean,
  editFname: boolean,
  flipped: string,
) {
  const source = payload.graph;
  const selectedAddr = payload.blockLoc || "filenode";
  const [renderedAddr, _] = useState("");
  const [fname, setFname] = useState(payload.filename);
  const [_nodePositions, setNodePositions] = useState<
    Map<string, { x: number; y: number }>
  >(new Map());

  // Handle node position changes (when nodes are dragged)
  const onNodesChange = useCallback((changes: any) => {
    changes.forEach((change: any) => {
      if (change.type === "position" && change.position) {
        setNodePositions((prev) => {
          const newPositions = new Map(prev);
          newPositions.set(change.id, change.position);
          return newPositions;
        });
      }
    });
  }, []);

  // Calculate positions with proper spacing
  const calculateNodePositions = (nodes: RTLNode[], startX: number = 0) => {
    const positionedNodes: Node[] = [];
    //let currentX = startX;
    const HORIZONTAL_PADDING = 50; // Minimum padding between nodes
    const VERTICAL_PADDING = 100; // Minimum padding between levels
    const MIN_NODE_WIDTH = 200; // Minimum assumed width for nodes
    const MIN_NODE_HEIGHT = 150; // Minimum assumed height for nodes

    // Helper function to estimate node width based on content
    const estimateNodeWidth = (node: RTLNode): number => {
      let width = MIN_NODE_WIDTH;

      // Base width for the node container
      width += 80; // Padding and borders

      const getPieceWidth = (piece: RTLPiece): number => {
        switch (piece) {
          case "NOTHING":
            return 8 * piece.length;
          case "PendingVal":
            return 40;
          case "PendingOp":
            return 40;
          default:
            if (piece.IDENT) {
              return (piece.IDENT as string).length * 8; // ~8px per character
            } else if (piece.NUMBER) {
              return 40; // Fixed width for numbers
            } else if (piece.TEXT) {
              return (piece.TEXT as string).length * 8; // ~8px per character
            } else if (piece.OP) {
              return 30; // Fixed width for operators
            } else if (piece.BOOL) {
              return 40; // Fixed width for booleans
            } else if (piece.LIST || piece.FNCALL) {
              return 60; // Fixed width for lists/functions
            }
        }

        return 0;
      };

      // Add width based on node pieces
      if (node.pieces && node.pieces.length > 0) {
        // Estimate width for each piece type
        node.pieces.forEach((piece: RTLPiece) => {
          width += getPieceWidth(piece);
        });
      }

      // Add width for children
      if (node.children && node.children.length > 0) {
        // Estimate space needed for child nodes
        const childWidth = node.children.length * 100; // Base width per child
        width = Math.max(width, childWidth);
      }

      // Add extra space for complex nodes
      if (node.kind === "CONDTLY" || node.kind === "CONDTLN") {
        width += 100; // Conditional nodes need more space
      }

      // Ensure minimum width
      return Math.max(width, MIN_NODE_WIDTH);
    };

    const runNode = (node: RTLNode, currentX: { x: number }, level: number) => {
      // Estimate node dimensions based on content
      const estimatedWidth = estimateNodeWidth(node);

      // Position the node
      const nodePosition = {
        x: currentX.x,
        y: level * (VERTICAL_PADDING + MIN_NODE_HEIGHT),
      };

      positionedNodes.push({
        id: node.address,
        type: "rtlNode",
        position: nodePosition,
        data: {
          node,
          selectedAddr,
          renderedAddr,
          pieceIx: payload.pieceIx,
          parentIndents: 0,
          flipped,
        },
      });

      // Update currentX for next node with padding
      currentX.x += estimatedWidth + HORIZONTAL_PADDING;
      node.children
        .filter((child) => child.kind === "FNDEF")
        .forEach((child) => runNode(child, { x: 0 }, level + 1));
    };

    let currX = { x: startX };
    nodes.forEach((node) => runNode(node, currX, 1));

    return positionedNodes;
  };

  // Convert RTL nodes to reactflow nodes
  const nodes = useMemo(() => {
    const flowNodes: Node[] = [];

    // Add file node
    flowNodes.push({
      id: "filenode",
      type: "fileNode",
      position: { x: 400, y: 50 },
      data: {
        fname,
        setFname,
        includeBorder: selectedAddr === "filenode",
        editing: editFname,
      },
    });

    // Position all RTL nodes with proper spacing
    const positionedRTLNodes = calculateNodePositions(source);
    flowNodes.push(...positionedRTLNodes);

    // Store positions for potential future use
    const newPositions = new Map<string, { x: number; y: number }>();
    positionedRTLNodes.forEach((node) => {
      newPositions.set(node.id, node.position);
    });
    setNodePositions(newPositions);

    return flowNodes;
  }, [
    source,
    selectedAddr,
    renderedAddr,
    payload.pieceIx,
    flipped,
    fname,
    setFname,
    editFname,
  ]);

  // Convert RTL relationships to reactflow edges
  const edges = useMemo(() => {
    const flowEdges = [
      ...source.map((node: RTLNode) => nodeToNode("filenode", node)),
      ...source.flatMap(parentToEdge),
    ];

    return flowEdges;
  }, [source]);

  return (
    <div style={{ display: hide ? "none" : "" }}>
      <div className="relative" style={{ height: "100vh" }}>
        <ReactFlowProvider>
          <ReactFlow
            nodes={nodes}
            edges={edges}
            nodeTypes={nodeTypes}
            fitView
            fitViewOptions={{
              padding: 0.1, // Add 10% padding around the view
              includeHiddenNodes: false,
            }}
            style={{ background: "transparent" }}
            defaultEdgeOptions={{
              type: "default",
              style: { stroke: "white", strokeWidth: 2 },
            }}
            proOptions={{ hideAttribution: true }}
            nodesDraggable={true}
            nodesConnectable={false}
            elementsSelectable={true}
            minZoom={0.1}
            maxZoom={2}
            onNodesChange={onNodesChange}
          >
            <Controls />
            <Background />
          </ReactFlow>
        </ReactFlowProvider>
      </div>
    </div>
  );
}

const nodeToNode = (parentAddr: string, child: RTLNode) => {
  return {
    id: `${parentAddr}-${child.address}`,
    source: parentAddr,
    target: child.address,
    sourceHandle: `${parentAddr}-source`,
    targetHandle: `${child.address}-target`,
    type: "default",
    style: { stroke: "white", strokeWidth: 2 },
  };
};

const parentToEdge = (node: RTLNode) => {
  const parentToNode = (child: RTLNode) => nodeToNode(node.address, child);
  return node.children
    .filter((child: RTLNode) => ARROW_NODES.includes(child.kind))
    .map(parentToNode);
};
