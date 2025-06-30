// Settings dialog component
import { useBoundStore } from '../../stores';

export const SettingsDialog = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  const { isVimMode, toggleVimMode, watchedFolders, addWatchedFolder } = useBoundStore();

  if (!isOpen) return null;

  const handleAddFolder = () => {
    // In a real app, this would open a native file dialog.
    // For now, we'll just add a mock path.
    addWatchedFolder(`/path/to/new/folder/${Date.now()}`);
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex justify-center items-center">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 w-full max-w-xl">
        <h3 className="text-lg font-bold mb-4">Settings</h3>
        
        <div className="space-y-6">
          <div>
            <h4 className="font-semibold mb-2">Editor</h4>
            <div className="flex items-center justify-between">
              <label htmlFor="vimMode">Enable Vim Keybindings</label>
              <input
                type="checkbox"
                id="vimMode"
                className="h-4 w-4 rounded"
                checked={isVimMode}
                onChange={toggleVimMode}
              />
            </div>
          </div>

          <div>
            <h4 className="font-semibold mb-2">Watched Folders</h4>
            <div className="p-4 border rounded bg-gray-50 dark:bg-gray-700 dark:border-gray-600">
              <ul className="mb-2">
                {watchedFolders.map(folder => <li key={folder} className="text-sm font-mono">{folder}</li>)}
              </ul>
              <button
                onClick={handleAddFolder}
                className="mt-2 px-3 py-1 text-sm rounded bg-blue-500 text-white hover:bg-blue-600"
              >
                Add Folder
              </button>
            </div>
          </div>
        </div>

        <div className="flex justify-end gap-2 mt-6">
          <button onClick={onClose} className="px-4 py-2 rounded bg-gray-200 hover:bg-gray-300 dark:bg-gray-600 dark:hover:bg-gray-500">
            Close
          </button>
        </div>
      </div>
    </div>
  );
}; 