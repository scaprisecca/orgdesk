import { create } from 'zustand';

interface SettingsState {
  watchedFolders: string[];
  todoStateConfig: {
    todo: string;
    done: string;
  };
  addWatchedFolder: (folderPath: string) => void;
  removeWatchedFolder: (folderPath: string) => void;
  setTodoStateConfig: (config: { todo: string; done: string }) => void;
}

export const useSettingsSlice = create<SettingsState>((set) => ({
  watchedFolders: [],
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
})); 