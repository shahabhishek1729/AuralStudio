export interface node {
  id: number;
  children: node[];
  pieces: piece[];
}

export interface piece {
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

/**
 * Flattens a nested, reecursive array of nodes into an arary with depth=1 
 * @param data The nested/recursive array of nodes 
 * @param level The depth of a given flatnode (should always be initialized to 1, will be changed 
 *              in recursive calls within the function)
 * @param parent The id of a flatnode's parent (should always be initialized to null, will be changed
 *				in recursive calls within the function)
 */
export function* flatten(data: node[], level: number = 1, parent: parent_id = null): IterableIterator<flatnode> {
  for (const elem of data) {
    yield { id: elem.id, level: level, parent: parent };
	// Recurse over the child nodes
    if (elem.children.length > 0) 
		yield* flatten(elem.children, level + 1, elem.id);
  }
}

