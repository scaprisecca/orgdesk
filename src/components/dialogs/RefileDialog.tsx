import { useState, useMemo } from 'react';
import Fuse from 'fuse.js';
import { useTasksSlice } from '../../stores';
import type { Task } from '../../stores';

/** Every distinct source file among currently loaded tasks — real backend
 * data instead of a hardcoded fake file list (see M7 in the code review). */
const collectFilePaths = (tasks: Task[]): string[] => {
  const paths = new Set<string>();
  const visit = (list: Task[]) => {
    for (const task of list) {
      paths.add(task.filePath);
      if (task.children) visit(task.children);
    }
  };
  visit(tasks);
  return Array.from(paths).sort();
};

export const RefileDialog = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  const [query, setQuery] = useState('');
  const tasks = useTasksSlice((state) => state.tasks);
  const refileTargets = useMemo(
    () => collectFilePaths(tasks).map((path) => ({ id: path, title: path })),
    [tasks],
  );
  const fuse = useMemo(
    () => new Fuse(refileTargets, { keys: ['title'], includeScore: true }),
    [refileTargets],
  );
  const results = useMemo(() => {
    if (!query) return [];
    return fuse.search(query);
  }, [query, fuse]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex justify-center items-start pt-20">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-4 w-full max-w-md">
        <input
          type="text"
          placeholder="Refile task to..."
          className="w-full p-2 border rounded bg-gray-50 dark:bg-gray-700 dark:border-gray-600"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          autoFocus
        />
        <div className="mt-2 text-sm text-gray-500">
          Start typing to search for a destination file.
        </div>
        <ul className="mt-4 max-h-60 overflow-y-auto">
          {results.map(({ item }) => (
            <li key={item.id} className="p-2 hover:bg-gray-100 dark:hover:bg-gray-700 rounded cursor-pointer">
              {item.title}
            </li>
          ))}
        </ul>
        <div className="flex justify-end gap-2 mt-4">
          <button onClick={onClose} className="px-4 py-2 rounded bg-gray-200 hover:bg-gray-300 dark:bg-gray-600 dark:hover:bg-gray-500">
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}; 