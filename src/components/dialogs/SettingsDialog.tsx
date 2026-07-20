import { open, save } from '@tauri-apps/plugin-dialog';
import { useUiSlice, useSettingsSlice } from '../../stores';
import { Dialog } from '../ui/Dialog';

export const SettingsDialog = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  const { isVimMode, toggleVimMode } = useUiSlice();
  const { watchedFolders, addWatchedFolder, removeWatchedFolder, inboxFile, setInboxFile } = useSettingsSlice();

  const handleAddFolder = async () => {
    const folder = await open({ directory: true, multiple: false });
    if (typeof folder === 'string') {
      await addWatchedFolder(folder);
    }
  };

  const handlePickInboxFile = async () => {
    const path = await save({
      title: 'Choose inbox file',
      defaultPath: inboxFile ?? 'inbox.org',
      filters: [{ name: 'Org files', extensions: ['org'] }],
    });
    if (path) {
      await setInboxFile(path);
    }
  };

  return (
    <Dialog isOpen={isOpen} onClose={onClose} title="Settings" maxWidthClassName="max-w-xl">
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
          <h4 className="font-semibold mb-2">Quick Capture Inbox</h4>
          <div className="p-4 border rounded bg-gray-50 dark:bg-gray-700 dark:border-gray-600 flex items-center justify-between gap-2">
            <span className="text-sm font-mono truncate">
              {inboxFile ?? 'No inbox file set — Quick Capture will fail until one is chosen'}
            </span>
            <button
              onClick={handlePickInboxFile}
              className="shrink-0 px-3 py-1 text-sm rounded bg-blue-500 text-white hover:bg-blue-600"
            >
              Choose
            </button>
          </div>
        </div>

        <div>
          <h4 className="font-semibold mb-2">Watched Folders</h4>
          <div className="p-4 border rounded bg-gray-50 dark:bg-gray-700 dark:border-gray-600">
            <ul className="mb-2 space-y-1">
              {watchedFolders.map(folder => (
                <li key={folder} className="flex items-center justify-between gap-2 text-sm font-mono">
                  <span className="truncate">{folder}</span>
                  <button
                    onClick={() => removeWatchedFolder(folder)}
                    className="shrink-0 text-xs text-red-600 hover:underline"
                  >
                    Remove
                  </button>
                </li>
              ))}
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
    </Dialog>
  );
}; 