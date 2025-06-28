import _ELK from "elkjs/lib/elk.bundled";

export const ELK_OPTIONS = {
"elk.algorithm": "mrtree",
"elk.layered.spacing.nodeNodeBetweenLayers": "100",
"elk.spacing.nodeNode": "80",
"elk.layered.considerModelOrder.strategy": "PREFER_EDGES",
"elk.layered.considerModelOrder": "true",
"elk.validateGraph": "true",
};

export const ELK = new _ELK();
