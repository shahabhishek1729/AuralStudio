export interface node {
  id: number;
  children: node[];
  blocks: block[];
}

export interface block {
  kind: string;
  name?: string | number | boolean;
  line: number;
}

export interface flatnode {
  id: number;
  parent: parent_id;
  level: number;
}

type parent_id = number | null;

function flatten(
  data: node[],
  level: number = 1,
  parent: parent_id = null,
  result: flatnode[] = []
): flatnode[] {
  for (let i = 0; i < data.length; i++) {
    result.push({
      id: data[i].id,
      level: level,
      parent: parent,
    });
    if (data[i].children.length > 0) {
      result.concat(flatten(data[i].children, level + 1, data[i].id, result));
    }
  }
  return result;
}

export default flatten;
