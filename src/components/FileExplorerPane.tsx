import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ChevronRight, ChevronDown, Folder, FileText } from 'lucide-react';
import { useBoundStore } from '../stores';

// Define the shape of the file system node from the backend
interface FsNode {
  name: string;
  path: string;
  is_dir: boolean;
  children?: FsNode[];
}

const File = ({ name, path }: { name: string; path: string }) => {
  const selectedFile = useBoundStore((state) => state.selectedFile);
  const setSelectedFile = useBoundStore((state) => state.setSelectedFile);
  const isSelected = selectedFile === path;

  return (
    <div
      className={`flex items-center gap-2 pl-4 cursor-pointer hover:bg-accent rounded text-sm ${
        isSelected ? 'bg-primary text-primary-foreground hover:bg-primary/90' : ''
      }`}
      onClick={() => setSelectedFile(path)}
    >
      <FileText size={16} />
      <span>{name}</span>
    </div>
  );
};

const Directory = ({ name, children }: FsNode) => {
  const [isOpen, setIsOpen] = useState(true);

  return (
    <div className="text-sm">
      <div
        className="flex items-center gap-2 cursor-pointer hover:bg-accent rounded"
        onClick={() => setIsOpen(!isOpen)}
      >
        {isOpen ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
        <Folder size={16} />
        <span className="font-semibold">{name}</span>
      </div>
      {isOpen && (
        <div className="pl-4">
          {children?.map((node) =>
            node.is_dir ? (
              <Directory key={node.path} {...node} />
            ) : (
              <File key={node.path} name={node.name} path={node.path} />
            )
          )}
        </div>
      )}
    </div>
  );
};

export const FileExplorerPane = () => {
  const [fileTree, setFileTree] = useState<FsNode[] | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchFileTree = async () => {
      try {
        const result = await invoke<FsNode>('read_fs', { dir: 'data' });
        setFileTree(result.children || []); // Start with the contents of the root
      } catch (err) {
        setError(String(err));
      }
    };

    fetchFileTree();
  }, []);

  return (
    <div className="h-full p-2 bg-background/50 flex flex-col min-h-0">
      <div className="p-2 mb-2 flex-shrink-0">
        <h2 className="text-lg font-bold text-foreground">Files</h2>
      </div>
      <div className="flex-grow overflow-y-auto space-y-1 pr-2">
        {error && <p className="text-red-500 p-2 bg-red-100 dark:bg-red-900 rounded">{error}</p>}
        {fileTree ? (
          fileTree.length > 0 ? (
            fileTree.map((node) =>
              node.is_dir ? (
                <Directory key={node.path} {...node} />
              ) : (
                <File key={node.path} name={node.name} path={node.path} />
              )
            )
          ) : (
            <p className="text-muted-foreground text-sm p-2">No .org files found in data directory</p>
          )
        ) : (
          <p className="text-muted-foreground text-sm p-2">Loading files...</p>
        )}
      </div>
    </div>
  );
}; 