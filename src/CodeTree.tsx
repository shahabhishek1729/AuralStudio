import React, { useRef, useLayoutEffect, useEffect } from "react";
import useResizeObserver from "./useResizeObserver";
import * as d3 from "d3";

function getY(node, maxDepth) {
  let mult = node.depth === maxDepth ? -1 : 1;
  return node.y + (node.depth > 0 && node.depth < maxDepth ? 0 : mult * 40);
}

function getNodeFill(node) {
  if (node.data.name.includes("variable")) return "blue";
  if (node.data.name.includes("function")) return "red";
  if (node.data.name.includes("class")) return "green";
  else return "black";
}

function getMaxNestedLevels(obj) {
  if (!obj || typeof obj !== "object" || !("children" in obj)) {
    return 0;
  }

  let maxDepth = 0;
  if (obj.children) {
    for (const child of obj.children) {
      const depth = 1 + getMaxNestedLevels(child);
      maxDepth = Math.max(maxDepth, depth);
    }
  }

  return maxDepth;
}

function TreeNode({ text }) {
  return <h1>Hello World</h1>;
}

function TreeChart({ data }) {
  const maxLevels = getMaxNestedLevels(data);
  const svgRef = useRef();
  const wrapperRef = useRef();
  const dimensions = useResizeObserver(wrapperRef);

  // useLayoutEffect(() => {
  //   window.addEventListener('resize', updateSize);
  //   updateSize();
  //   return () => window.removeEventListener('resize', updateSize);
  // }, []);

  useEffect(() => {
    const svg = d3.select(svgRef.current);
    if (!dimensions) return;

    const root = d3.hierarchy(data);
    const treeLayout = d3
      .tree()
      .size([dimensions.width, dimensions.height])
      .separation((a, b) => (a.parent == b.parent ? 15 : 10));
    treeLayout(root);

    console.log("The root descentants are:");
    console.log(root.descendants());
    console.log(root.links());

    const linkGen = d3
      .linkVertical()
      .y((node) => getY(node, maxLevels))
      .x((node) => node.x);

    svg
      .selectAll(".link")
      .data(root.links())
      .join("path")
      .attr("class", "link")
      .attr("fill", "none")
      .attr("stroke", "white")
      .attr("d", linkGen);

    svg
      .selectAll(".node")
      .data(root.descendants())
      .join("g")
      .attr("class", "node")
      .attr(
        "transform",
        (node) => `translate(${node.x - 125}, ${getY(node, maxLevels) - 35})`
      )
      .append("TreeNode");

    // svg
    //   .selectAll(".node")
    //   .data(root.descendants())
    //   .join("rect")
    //   .attr("class", "node")
    //   .attr("width", 250)
    //   .attr("height", 40)
    //   .attr("rx", 10)
    //   // .attr("r", 45)
    //   .attr("fill", getNodeFill)
    //   .attr("y", node => getY(node, maxLevels) - 35)
    //   .attr("x", node => node.x - 125)

    // svg
    //   .selectAll(".label")
    //   .data(root.descendants())
    //   .join("text")
    //   .attr("class", "label")
    //   .text(node => node.data.name)
    //   .attr("text-anchor", "middle")
    //   .attr("fill", "white")
    //   .attr("font-size", 20)
    //   .attr("font-family", "Andale Mono")
    //   .attr("font-weight", "bold")
    //   .attr("y", node => getY(node, maxLevels))
    //   .attr("x", node => node.x + 6);
  }, [data, dimensions]);

  return (
    <div ref={wrapperRef} style={{ width: "100%", height: "100%" }}>
      <svg ref={svgRef} style={{ height: "100%", width: "100%" }}></svg>
    </div>
  );
}

// function Box() {
// return <p style={{width: "260px", backgroundColor: "red", borderRadius: "10px", fontSize: "24px", padding: "10px 20px"}}><b>function</b> add_numbers()</p>
// }

export default TreeChart;
