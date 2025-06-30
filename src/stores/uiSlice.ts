import type { StateCreator } from 'zustand';
import type { TasksSlice } from './tasksSlice';
import type { SettingsSlice } from './settingsSlice';
import type { AgendaSlice } from './agendaSlice';

export type ModalType = 'QuickCapture' | 'Refile' | 'AgendaBuilder' | 'Settings' | null;
export type RightPaneView = 'Agenda' | 'Note';

export interface UiSlice {
  activeModal: ModalType;
  isVimMode: boolean;
  paneSizes: number[];
  selectedFile: string | null;
  rightPaneView: RightPaneView;
  secondarySelectedFile: string | null;
  openModal: (modal: ModalType) => void;
  closeModal: () => void;
  toggleVimMode: () => void;
  setPaneSizes: (sizes: number[]) => void;
  setSelectedFile: (path: string) => void;
  setRightPaneView: (view: RightPaneView) => void;
  setSecondarySelectedFile: (path: string | null) => void;
}

export const createUiSlice: StateCreator<
  UiSlice & TasksSlice & SettingsSlice & AgendaSlice,
  [],
  [],
  UiSlice
> = (set) => ({
  activeModal: null,
  isVimMode: false,
  paneSizes: [25, 45, 30], // For FileExplorer, TaskList, Agenda
  selectedFile: null,
  rightPaneView: 'Agenda',
  secondarySelectedFile: null,
  openModal: (modal) => set({ activeModal: modal }),
  closeModal: () => set({ activeModal: null }),
  toggleVimMode: () => set((state) => ({ isVimMode: !state.isVimMode })),
  setPaneSizes: (sizes) => set({ paneSizes: sizes }),
  setSelectedFile: (path) => set({ selectedFile: path }),
  setRightPaneView: (view) => set({ rightPaneView: view }),
  setSecondarySelectedFile: (path) => set({ secondarySelectedFile: path }),
}); 