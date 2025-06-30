import React from 'react';
import * as DropdownMenu from '@radix-ui/react-dropdown-menu';

export const Dropdown = ({
  trigger,
  children,
}: {
  trigger: React.ReactNode;
  children: React.ReactNode;
}) => {
  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>{trigger}</DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content
          className="min-w-[220px] bg-white dark:bg-gray-800 rounded-md p-1 shadow-lg border border-gray-200 dark:border-gray-700"
          sideOffset={5}
        >
          {children}
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  );
};

export const DropdownItem = ({
  children,
  onSelect,
}: {
  children: React.ReactNode;
  onSelect?: () => void;
}) => {
  return (
    <DropdownMenu.Item
      className="text-[13px] leading-none rounded-[3px] flex items-center h-[25px] px-[5px] relative pl-[25px] select-none outline-none text-gray-900 dark:text-gray-100 data-[highlighted]:bg-blue-500 data-[highlighted]:text-white"
      onSelect={onSelect}
    >
      {children}
    </DropdownMenu.Item>
  );
}; 