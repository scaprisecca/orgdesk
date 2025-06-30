import React from 'react';
import { Plus, Settings, MoreHorizontal } from 'lucide-react';
import { useUiSlice } from '../stores';
import { Dropdown, DropdownItem } from './ui/Dropdown';

export const Toolbar = () => {
  const { openModal } = useUiSlice();

  return (
    <div className="flex items-center justify-between p-2 bg-gray-200 dark:bg-gray-800 border-b border-gray-300 dark:border-gray-700">
      <div className="flex items-center gap-2">
        <span className="font-semibold text-lg">OrgDesk</span>
      </div>
      <div className="flex items-center gap-2">
        <button
          onClick={() => openModal('QuickCapture')}
          className="p-1.5 hover:bg-gray-300 dark:hover:bg-gray-700 rounded"
          aria-label="Quick Capture"
        >
          <Plus size={18} />
        </button>
        <button
          onClick={() => openModal('Settings')}
          className="p-1.5 hover:bg-gray-300 dark:hover:bg-gray-700 rounded"
          aria-label="Settings"
        >
          <Settings size={18} />
        </button>
        <Dropdown
          trigger={
            <button className="p-1.5 hover:bg-gray-300 dark:hover:bg-gray-700 rounded" aria-label="More options">
              <MoreHorizontal size={18} />
            </button>
          }
        >
          <DropdownItem onSelect={() => console.log('File > New')}>File &gt; New</DropdownItem>
          <DropdownItem>File &gt; Open</DropdownItem>
          <DropdownItem>Edit &gt; Undo</DropdownItem>
          <DropdownItem>Help &gt; About</DropdownItem>
        </Dropdown>
      </div>
    </div>
  );
}; 