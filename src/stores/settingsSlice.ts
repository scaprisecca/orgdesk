import type { StateCreator } from 'zustand';
import type { TasksSlice } from './tasksSlice';
import type { AgendaSlice } from './agendaSlice';
import type { UiSlice } from './uiSlice';

export interface SettingsSlice {
  watchedFolders: string[];
  todoStateConfig: {
    todo: string;
    done: string;
  };
  addWatchedFolder: (folderPath: string) => void;
  removeWatchedFolder: (folderPath: string) => void;
  setTodoStateConfig: (config: { todo: string; done: string }) => void;
}

export const createSettingsSlice: StateCreator<
  SettingsSlice & TasksSlice & AgendaSlice & UiSlice,
  [],
  [],
  SettingsSlice
> = (set) => ({
  watchedFolders: ['./data'],
  todoStateConfig: {
    todo: 'TODO',
    done: 'DONE',
  },
  addWatchedFolder: (folderPath) =>
    set((state) => ({
      watchedFolders: [...state.watchedFolders, folderPath],
    })),
  removeWatchedFolder: (folderPath) =>
    set((state) => ({
      watchedFolders: state.watchedFolders.filter((f) => f !== folderPath),
    })),
  setTodoStateConfig: (config) => set({ todoStateConfig: config }),
}); 