import { useState, useEffect } from 'react';
import { useBoundStore } from '../stores';
import { invoke } from '@tauri-apps/api/core';
import { parse } from 'org';
import type { Root } from 'org';
import { OrgNode } from './OrgNode';

export const TaskListPane = () => {
  const selectedFile = useBoundStore((state) => state.selectedFile);
  const [ast, setAst] = useState<Root | null>(null);
  const [error, setError] = useState('');

  useEffect(() => {
    if (!selectedFile) {
      setAst(null);
      return;
    }

    const fetchContent = async () => {
      try {
        const content = await invoke<string>('get_file_content', { path: selectedFile });
        const parsed = parse(content);
        setAst(parsed);
        setError('');
      } catch (err) {
        setError(String(err));
        setAst(null);
      }
    };

    fetchContent();
  }, [selectedFile]);

  return (
    <div className="h-full p-4 bg-background flex flex-col text-foreground min-h-0">
      <div className="mb-4 flex-shrink-0">
        <h2 className="text-xl font-bold text-foreground">
          {selectedFile ? selectedFile.split('/').pop() : 'Task List'}
        </h2>
      </div>
      <div className="flex-grow overflow-y-auto pr-4">
        {selectedFile ? (
          error ? (
            <p className="text-destructive-foreground">{error}</p>
          ) : ast ? (
            <OrgNode node={ast} />
          ) : null
        ) : (
          <p className="text-center text-muted-foreground pt-10">Select a file to view its content.</p>
        )}
      </div>
    </div>
  );
}; 