import { useState, useMemo } from 'react';
import Fuse from 'fuse.js';
// Task type imported but handled by other imports

// Mock data for refile targets for now. In a real app, this would
// likely come from a list of files or major headlines.
const refileTargets = [
  { id: 'file1', title: 'work.org' },
  { id: 'file2', title: 'home.org' },
  { id: 'file3', title: 'project-alpha.org' },
];

const fuse = new Fuse(refileTargets, {
  keys: ['title'],
  includeScore: true,
});

export const RefileDialog = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  const [query, setQuery] = useState('');
  const results = useMemo(() => {
    if (!query) return [];
    return fuse.search(query);
  }, [query]);

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