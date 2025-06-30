// Component for rendering org-mode AST nodes
import type { Node, Parent } from 'org';

export const OrgNode = ({ node }: { node: Node }) => {
  if (!node || !node.type) {
    return null;
  }

  const children = (node as Parent).children?.map((child: Node, i: number) => (
    <OrgNode key={i} node={child} />
  ));

  switch (node.type) {
    case 'root':
      return <div className="space-y-4">{children}</div>;
    case 'headline':
      switch (node.level) {
        case 1:
          return (
            <h1 className="text-2xl font-bold border-b border-border pb-1 mb-2">
              {children}
            </h1>
          );
        case 2:
          return (
            <h2 className="text-xl font-bold border-b border-border pb-1 mb-2">
              {children}
            </h2>
          );
        case 3:
          return <h3 className="text-lg font-bold">{children}</h3>;
        case 4:
          return <h4 className="text-md font-bold">{children}</h4>;
        case 5:
          return <h5 className="text-sm font-bold">{children}</h5>;
        default:
          return <h6 className="text-xs font-bold">{children}</h6>;
      }
    case 'paragraph':
      return <p className="mb-2">{children}</p>;
    case 'list':
      return <ul className="list-disc pl-6 space-y-1 mb-2">{children}</ul>;
    case 'item':
      return <li>{children}</li>;
    case 'text':
      return <span>{node.value as string}</span>;
    default:
      return null; // Don't render unknown nodes
  }
}; 