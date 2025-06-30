import { useState } from 'react';
import { useBoundStore } from '../../stores';

export const QuickCaptureModal = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  const [title, setTitle] = useState('');
  const addTask = useBoundStore((state) => state.addTask);

  if (!isOpen) return null;

  const handleSubmit = () => {
    if (title.trim()) {
      addTask({
        id: new Date().toISOString(), // Temporary ID
        title: title.trim(),
        state: 'TODO',
      });
      setTitle('');
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex justify-center items-center">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 w-full max-w-lg">
        <h3 className="text-lg font-bold mb-4">Quick Capture</h3>
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
        ></textarea>
        <div className="flex justify-end gap-2 mt-4">
          <button onClick={onClose} className="px-4 py-2 rounded bg-gray-200 hover:bg-gray-300 dark:bg-gray-600 dark:hover:bg-gray-500">
            Cancel
          </button>
          <button onClick={handleSubmit} className="px-4 py-2 rounded bg-blue-500 text-white hover:bg-blue-600">
            Add Task
          </button>
        </div>
      </div>
    </div>
  );
}; 