/**
 * @fileoverview Renders the acyclic digraph for a given source file. Handles the rendering of
 * individual pieces, nodes, blocks and subtrees.
 */

import {
  useState,
  useMemo,
  useCallback,
  useLayoutEffect,
  useEffect,
} from "react";
import ReactFlow, {
  Controls,
  Background,
  ReactFlowProvider,
  useReactFlow,
  useEdgesState,
  useNodesState,
} from "reactflow";
import "reactflow/dist/style.css";
import { Canvas, RTLNode } from "./typing/digraph";
import { nodeTypes } from "./typing/nodeTypes";
import { ELK, ELK_OPTIONS } from "./constants";
import type { Node, Edge } from "./typing/reactflow";

// Global state to store node sizes from ResizeObserver
const nodeSizes = new Map<string, { width: number; height: number }>();

// Function to update node size (will be called from node components)
export const updateNodeSize = (
  nodeId: string,
  width: number,
  height: number,
) => {
  nodeSizes.set(nodeId, { width, height });
  // Trigger a layout update when sizes change
  if (window.layoutUpdateCallback) {
    window.layoutUpdateCallback();
  }
};

// Add callback to window for global access
declare global {
  interface Window {
    layoutUpdateCallback?: () => void;
  }
}

function ChildDAG({
  payload,
  editFname,
  flipped,
}: {
  payload: any;
  editFname: any;
  flipped: any;
}) {
  const source = payload.graph;
  const selectedAddr = payload.blockLoc || "filenode";
  const [renderedAddr, _] = useState("");
  const [fname, setFname] = useState(payload.filename);
  const [layoutVersion, setLayoutVersion] = useState(0);

  const [initialLaunch, setInitialLaunch] = useState(0);

  // Set up global callback for layout updates
  useLayoutEffect(() => {
    window.layoutUpdateCallback = () => {
      setLayoutVersion((prev) => prev + 1);
    };

    return () => {
      delete window.layoutUpdateCallback;
    };
  }, []);

  const getLayoutedElements = async (
    nodes: Node[],
    edges: Edge[],
    options: any = {},
  ) => {
    const isHorizontal = options?.["elk.direction"] === "RIGHT";
    const graph = {
      id: "root",
      layoutOptions: options,
      children: nodes.map((node) => {
        // Get dynamic size from ResizeObserver, fallback to defaults
        const dynamicSize = nodeSizes.get(node.id) || {
          width: 150,
          height: 50,
        };

        return {
          ...node,
          // Adjust the target and source handle positions based on the layout
          // direction.
          targetPosition: isHorizontal ? "left" : "top",
          sourcePosition: isHorizontal ? "right" : "bottom",

          // Use dynamic width and height from ResizeObserver
          width: dynamicSize.width,
          height: dynamicSize.height,
        };
      }),
      edges: edges.map((edge) => ({
        ...edge,
        sources: [edge.source],
        targets: [edge.target],
      })),
    };

    return ELK.layout(graph)
      .then((layoutedGraph) => ({
        nodes:
          layoutedGraph.children?.map((node) => ({
            id: node.id,
            type: node.type,
            position: { x: node.x || 0, y: node.y || 0 },
            data: node.data,
            style: node.style,
          })) || [],

        edges: layoutedGraph.edges || [],
      }))
      .catch(console.error);
  };

  const [nodes_, setNodes_, onNodesChange_] = useNodesState([]);
  const [edges_, setEdges_, onEdgesChange_] = useEdgesState([]);
  const { fitView, getViewport, setViewport, getNode } = useReactFlow();

  // Function to check if SlidingBorder is visible and scroll to it if needed
  const scrollToSlidingBorder = useCallback(() => {
    if (!selectedAddr) return;

    // Try to find the target element (same logic as SlidingBorder)
    let targetElement: HTMLElement | null = null;

    // Pattern 1: Direct selectedAddr match
    targetElement = document.getElementById(selectedAddr);

    // Pattern 2: selectedAddr with pieceIx (for input elements)
    if (!targetElement && payload.pieceIx) {
      const pieceIxStr = payload.pieceIx.join(",");
      const id = `${selectedAddr},${pieceIxStr}`;
      targetElement = document.getElementById(id);
    }

    // Pattern 3: selectedAddr with pieceIx array (alternative format)
    if (!targetElement && payload.pieceIx) {
      const id = `${selectedAddr},${payload.pieceIx}`;
      targetElement = document.getElementById(id);
    }

    // Pattern 4: selectedAddr with index (for RenderIdent)
    if (!targetElement) {
      const id = `${selectedAddr},0`;
      targetElement = document.getElementById(id);
    }

    // Pattern 5: selectedAddr with .0 suffix (for nodes)
    if (!targetElement) {
      const id = `${selectedAddr}.0`;
      targetElement = document.getElementById(id);
    }

    // Pattern 6: selectedAddr without .0 suffix (for parent nodes)
    if (!targetElement && selectedAddr.includes(".")) {
      const id = selectedAddr.slice(0, selectedAddr.length - 2);
      targetElement = document.getElementById(id);
    }

    // Pattern 7: selectedAddr with selected_ prefix (existing system)
    if (!targetElement) {
      const id = `selected_${selectedAddr}`;
      targetElement = document.getElementById(id);
    }

    if (targetElement) {
      const rect = targetElement.getBoundingClientRect();
      
      // Check if element is outside the visible area
      const elementCenterX = rect.left + rect.width / 2;
      const elementCenterY = rect.top + rect.height / 2;
      
      const viewportWidth = window.innerWidth;
      const viewportHeight = window.innerHeight;
      
      // Only scroll if the element is significantly outside the viewport
      const margin = 100; // pixels from edge to trigger scroll
      const shouldScrollX = elementCenterX < margin || elementCenterX > viewportWidth - margin;
      const shouldScrollY = elementCenterY < margin || elementCenterY > viewportHeight - margin;
      
      if (shouldScrollX || shouldScrollY) {
        // Simple approach: calculate the offset needed to center the element
        const offsetX = elementCenterX - viewportWidth / 2;
        const offsetY = elementCenterY - viewportHeight / 2;
        
        const viewport = getViewport();
        
        setViewport({
          x: viewport.x - offsetX,
          y: viewport.y - offsetY,
          zoom: viewport.zoom,
        }, { duration: 300 });
      }
    }
  }, [selectedAddr, payload.pieceIx, getViewport, setViewport]);

  // Calculate positions with proper spacing
  const calculateNodePositions = (source: RTLNode[]) => {
    const positionedNodes: Node[] = [];
    const runNode = (node: RTLNode) => {
      positionedNodes.push({
        id: node.address,
        type: "rtlNode",
        position: { x: 0, y: 0 },
        data: {
          node,
          selectedAddr,
          renderedAddr,
          pieceIx: payload.pieceIx,
          parentIndents: 0,
          flipped,
        },
      });

      node.children
        .filter((child) => child.kind === "FNDEF")
        .forEach((child) => runNode(child));
    };

    source.forEach((node) => runNode(node));
    return positionedNodes;
  };

  // Convert RTL nodes to reactflow nodes
  const nodes = useMemo(() => {
    const flowNodes: Node[] = calculateNodePositions(source);

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

  const onLayout = useCallback(
    ({
      direction,
      useInitialNodes = false,
    }: {
      direction: string;
      useInitialNodes?: boolean;
    }) => {
      const opts = { "elk.direction": direction, ...ELK_OPTIONS };
      const ns = useInitialNodes ? nodes : nodes_;
      const es = useInitialNodes ? edges : edges_;

      getLayoutedElements(ns, es, opts).then((result) => {
        if (result) {
          const { nodes: layoutedNodes, edges: layoutedEdges } = result;
          setNodes_(layoutedNodes);
          setEdges_(layoutedEdges);
          // Only fit view on initial layout, not on subsequent updates
          if (initialLaunch < 2) {
            console.log("Fitting!");
            fitView();
            setInitialLaunch(initialLaunch + 1);
          }
        }
      });
    },
    [nodes, edges],
  );

  useLayoutEffect(() => {
    onLayout({ direction: "DOWN", useInitialNodes: true });
  }, [layoutVersion, onLayout]);

  // Auto-scroll to SlidingBorder when selectedAddr changes
  useEffect(() => {
    // Add a small delay to ensure the DOM has updated
    const timeoutId = setTimeout(() => {
      scrollToSlidingBorder();
    }, 100);

    return () => clearTimeout(timeoutId);
  }, [selectedAddr, payload.pieceIx, scrollToSlidingBorder]);

  return (
    <ReactFlow
      nodes={nodes_}
      edges={edges_}
      nodeTypes={nodeTypes}
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
      fitView
      fitViewOptions={{
        padding: 0.1,
        includeHiddenNodes: false,
        duration: 10,
      }}
      onNodesChange={onNodesChange_}
      onEdgesChange={onEdgesChange_}
    >
      <Controls />
      <Background />
    </ReactFlow>
  );
}

export function DAG(
  payload: Canvas,
  hide: boolean,
  editFname: boolean,
  flipped: string,
) {
  return (
    <div style={{ display: hide ? "none" : "" }}>
      <div className="relative" style={{ height: "100vh" }}>
        <ReactFlowProvider>
          <ChildDAG payload={payload} editFname={editFname} flipped={flipped} />
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
