declare module 'org' {
  export interface Node {
    type: string;
    [key: string]: any;
  }

  export interface Parent extends Node {
    children: Node[];
  }

  export interface Root extends Parent {
    type: 'root';
  }

  export function parse(text: string): Root;
} 