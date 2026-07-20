import { useState } from 'react';
import { useTasksSlice } from '../../stores';
import { Dialog } from '../ui/Dialog';

export const QuickCaptureModal = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  const [title, setTitle] = useState('');
  const addTask = useTasksSlice((state) => state.addTask);

  const handleSubmit = async () => {
    if (title.trim()) {
      const capturedTitle = title.trim();
      setTitle('');
      onClose();
      await addTask(capturedTitle);
    }
  };

  return (
    <Dialog isOpen={isOpen} onClose={onClose} title="Quick Capture">
      <textarea
        className="w-full p-2 border rounded bg-gray-50 dark:bg-gray-700 dark:border-gray-600"
        rows={4}
        placeholder="Enter a new task..."
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
            handleSubmit();
          }
        }}
        autoFocus
      ></textarea>
      <div className="flex justify-end gap-2 mt-4">
        <button onClick={onClose} className="px-4 py-2 rounded bg-gray-200 hover:bg-gray-300 dark:bg-gray-600 dark:hover:bg-gray-500">
          Cancel
        </button>
        <button onClick={handleSubmit} className="px-4 py-2 rounded bg-blue-500 text-white hover:bg-blue-600">
          Add Task
        </button>
      </div>
    </Dialog>
  );
}; 