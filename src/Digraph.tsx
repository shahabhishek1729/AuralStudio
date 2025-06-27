/**
 * @fileoverview Renders the acyclic digraph for a given source file. Handles the rendering of
 * individual pieces, nodes, blocks and subtrees.
 */

import { useState, useMemo, useCallback } from "react";
import ReactFlow, {
  Node,
  Controls,
  Background,
  ReactFlowProvider,
  Edge,
} from "reactflow";
import "reactflow/dist/style.css";
import { Canvas, RTLNode, RTLPiece, nodeTypes } from "./types";

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

  // An edge represents a connection between blocks
  const edges = useMemo(() => {
    return [
      ...source.map((node: RTLNode) => nodeToNode("filenode", node)),
      ...source.flatMap(parentToEdge),
    ];
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

const nodeToNode = (parentAddr: string, child: RTLNode): Edge => {
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

const parentToEdge = (node: RTLNode): Edge[] => {
  const parentToNode = (child: RTLNode) => nodeToNode(node.address, child);
  let fnChildren = node.children.filter(
    (child: RTLNode) => child.kind === "FNDEF",
  );
  return [
    ...fnChildren.map(parentToNode),
    ...fnChildren.flatMap((child: RTLNode) => parentToEdge(child)),
  ];
};
