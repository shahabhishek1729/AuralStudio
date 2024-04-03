import React, { useMemo } from "react";
import { Group } from "@visx/group";
import { Tree, hierarchy } from "@visx/hierarchy";
import { HierarchyPointNode } from "@visx/hierarchy/lib/types";
import { LinkHorizontal } from "@visx/shape";
import { LinearGradient } from "@visx/gradient";
import hammer from "./assets/hammer.png";

const peach = "#fd9b93";
const pink = "#fe6e9e";
const blue = "#03c0dc";
const green = "#26deb0";
const plum = "#71248e";
const lightpurple = "#374469";
const white = "#ffffff";
export const background = "#272b4d";

interface TreeNode {
  name: string;
  children?: this[];
}

type HierarchyNode = HierarchyPointNode<TreeNode>;

const rawTree = {
  name: "file dog.py",
  children: [
    {
      name: "variable silent_dogs",
    },
    {
      name: "function greet()",
      children: [
        {
          name: "variable greeting",
        },
      ],
    },
    {
      name: "class Dog",
      children: [
        {
          name: "function __init__",
          children: [],
        },
        {
          name: "function bark()",
          children: [],
        },
      ],
    },
  ],
};

function RootNode({ node }: { node: HierarchyNode }) {
  const width = 140;
  const height = 40;
  const centerX = -width / 2;
  const centerY = -height / 2 - 4;

  return (
    <Group top={node.x} left={node.y}>
      <rect
        width={width}
        height={height}
        x={centerX}
        y={centerY}
        rx={10}
        fill="#ffffff"
      />
      <text
        dy=".33em"
        fontSize={20}
        fontFamily="Inter"
        textAnchor="middle"
        style={{ pointerEvents: "none" }}
        fill={"black"}
      >
        {node.data.name}
      </text>
    </Group>
  );
}

function ParentNode({ node }: { node: HierarchyNode }) {
  const width = 200;
  const height = 35;
  const centerX = -width / 2;
  const centerY = -height / 2 - 3;

  return (
    <Group top={node.x} left={node.y - 100}>
      <rect
        height={height}
        width={width}
        y={centerY}
        x={centerX}
        rx={10}
        fill={blue}
        strokeWidth={1}
        onClick={() => {
          alert(`clicked: ${JSON.stringify(node.data.name)}`);
        }}
      />
      <text
        dy=".33em"
        fontSize={16}
        fontFamily="JetBrains Mono"
        textAnchor="middle"
        style={{ pointerEvents: "none" }}
        fill={white}
      >
        {node.data.name}
      </text>
    </Group>
  );
}

/** Handles rendering Root, Parent, and other Nodes. */
function Node({ node }: { node: HierarchyNode }) {
  const width = 210;
  const height = 47;
  const centerX = -width / 2;
  const centerY = -height / 2;
  const isRoot = node.depth === 0;
  const isParent = !!node.children;

  if (isRoot) return <RootNode node={node} />;
  if (isParent) return <ParentNode node={node} />;

  return (
    <Group top={node.x} left={node.y}>
      <rect
        height={height}
        width={width}
        y={centerY}
        x={centerX}
        fill={green}
        strokeWidth={1}
        strokeDasharray="2,2"
        strokeOpacity={0.6}
        rx={10}
        onClick={() => {
          alert(`clicked: ${JSON.stringify(node.data.name)}`);
        }}
      />
      {/* <div
        style={{
          width: "20px",
          height: "20px",
          display: "flex",
          flexDirection: "row",
          backgroundImage: 'url("./assets/hammer.png")',
          backgroundColor: "black",
          marginRight: "0px",
          alignItems: "center",
        }}
      /> */}
      <img src={hammer} />
      <text
        dy=".33em"
        fontSize={16}
        fontFamily="JetBrains Mono"
        textAnchor="middle"
        fill={white}
        style={{ pointerEvents: "none" }}
      >
        {node.data.name}
      </text>
    </Group>
  );
}

const defaultMargin = { top: 10, left: 80, right: 80, bottom: 10 };

export type TreeProps = {
  width: number;
  height: number;
  margin?: { top: number; right: number; bottom: number; left: number };
};

export function Example({ width, height, margin = defaultMargin }: TreeProps) {
  const data = useMemo(() => hierarchy(rawTree), []);
  const yMax = height - margin.top - margin.bottom;
  const xMax = width - margin.left - margin.right;

  return width < 10 ? null : (
    <svg width={width} height={height}>
      <LinearGradient id="lg" from={peach} to={pink} />
      <rect width={width} height={height} rx={14} fill={background} />
      <Tree<TreeNode> root={data} size={[yMax, xMax]}>
        {(tree) => (
          <Group top={margin.top} left={margin.left}>
            {tree.links().map((link, i) => (
              <LinkHorizontal
                key={`link-${i}`}
                data={link}
                stroke={"#CCCCCC"}
                strokeWidth="2"
                fill="none"
              />
            ))}
            {tree.descendants().map((node, i) => (
              <Node key={`node-${i}`} node={node} />
            ))}
          </Group>
        )}
      </Tree>
    </svg>
  );
}
