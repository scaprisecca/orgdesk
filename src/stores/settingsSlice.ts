import { create } from 'zustand';
import {
  addWatchedFolder as apiAddWatchedFolder,
  removeWatchedFolder as apiRemoveWatchedFolder,
  getWatchedFolders,
} from '../lib/api';

interface SettingsState {
  watchedFolders: string[];
  todoStateConfig: {
    todo: string;
    done: string;
  };
  fetchWatchedFolders: () => Promise<void>;
  addWatchedFolder: (folderPath: string) => Promise<void>;
  removeWatchedFolder: (folderPath: string) => Promise<void>;
  setTodoStateConfig: (config: { todo: string; done: string }) => void;
}

export const useSettingsSlice = create<SettingsState>((set) => ({
  watchedFolders: [],
  todoStateConfig: {
    todo: 'TODO',
    done: 'DONE',
  },
  fetchWatchedFolders: async () => {
    const watchedFolders = await getWatchedFolders();
    set({ watchedFolders });
  },
  addWatchedFolder: async (folderPath) => {
    const watchedFolders = await apiAddWatchedFolder(folderPath);
    set({ watchedFolders });
  },
  removeWatchedFolder: async (folderPath) => {
    const watchedFolders = await apiRemoveWatchedFolder(folderPath);
    set({ watchedFolders });
  },
  setTodoStateConfig: (config) => set({ todoStateConfig: config }),
}));
