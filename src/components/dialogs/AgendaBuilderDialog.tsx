import React, { useState } from 'react';
import { useAgendaSlice } from '../../stores';

export const AgendaBuilderDialog = ({ isOpen, onClose }: { isOpen: boolean, onClose: () => void }) => {
  const [name, setName] = useState('');
  const addPreset = useAgendaSlice((state: any) => state.addPreset);

  if (!isOpen) return null;

  const handleSave = () => {
    if (name.trim()) {
      addPreset({
        id: new Date().toISOString(),
        name: name.trim(),
      });
      setName('');
      onClose();
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex justify-center items-center">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 w-full max-w-2xl">
        <h3 className="text-lg font-bold mb-4">Agenda Preset Builder</h3>
        
        <div className="space-y-4">
          <div>
            <label htmlFor="presetName" className="block text-sm font-medium">Preset Name</label>
            <input
              type="text"
              id="presetName"
              className="w-full p-2 mt-1 border rounded bg-gray-50 dark:bg-gray-700 dark:border-gray-600"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., 'Work Focus'"
            />
          </div>
          <div>
            <label className="block text-sm font-medium">Conditions</label>
            <div className="p-4 mt-1 border rounded bg-gray-50 dark:bg-gray-700 dark:border-gray-600">
              <p className="text-sm text-gray-500">Filters for tags, priorities, etc. will be configured here.</p>
            </div>
          </div>
        </div>

        <div className="flex justify-end gap-2 mt-6">
          <button onClick={onClose} className="px-4 py-2 rounded bg-gray-200 hover:bg-gray-300 dark:bg-gray-600 dark:hover:bg-gray-500">
            Cancel
          </button>
          <button onClick={handleSave} className="px-4 py-2 rounded bg-blue-500 text-white hover:bg-blue-600">
            Save Preset
          </button>
        </div>
      </div>
    </div>
  );
}; 