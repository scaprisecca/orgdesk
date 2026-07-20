import type { ReactNode } from 'react';
import * as RadixDialog from '@radix-ui/react-dialog';

const POSITION_CLASSNAME = {
  // Vertically and horizontally centered.
  center: 'top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2',
  // Anchored near the top, centered horizontally — for search-style dialogs
  // (e.g. RefileDialog) that shouldn't cover the middle of the screen.
  top: 'top-20 left-1/2 -translate-x-1/2',
} as const;

/**
 * Shared modal chrome built on `@radix-ui/react-dialog` instead of a plain
 * fixed `<div>`, so every dialog gets a focus trap, Escape-to-close, scroll
 * lock, and correct aria roles for free (see L1 in the code review — the
 * hand-rolled modals had none of these).
 */
export const Dialog = ({
  isOpen,
  onClose,
  title,
  children,
  maxWidthClassName = 'max-w-lg',
  position = 'center',
}: {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: ReactNode;
  maxWidthClassName?: string;
  position?: keyof typeof POSITION_CLASSNAME;
}) => {
  return (
    <RadixDialog.Root open={isOpen} onOpenChange={(open) => !open && onClose()}>
      <RadixDialog.Portal>
        <RadixDialog.Overlay className="fixed inset-0 bg-black/50 z-50" />
        <RadixDialog.Content
          className={`fixed z-50 w-full ${maxWidthClassName} ${POSITION_CLASSNAME[position]} bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 outline-none`}
        >
          <RadixDialog.Title className="text-lg font-bold mb-4">{title}</RadixDialog.Title>
          {children}
        </RadixDialog.Content>
      </RadixDialog.Portal>
    </RadixDialog.Root>
  );
};
