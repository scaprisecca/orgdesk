import { create } from 'zustand';
import {
  addWatchedFolder as apiAddWatchedFolder,
  removeWatchedFolder as apiRemoveWatchedFolder,
  getWatchedFolders,
  getInboxFile,
  setInboxFile as apiSetInboxFile,
} from '../lib/api';

interface SettingsState {
  watchedFolders: string[];
  inboxFile: string | null;
  todoStateConfig: {
    todo: string;
    done: string;
  };
  fetchWatchedFolders: () => Promise<void>;
  addWatchedFolder: (folderPath: string) => Promise<void>;
  removeWatchedFolder: (folderPath: string) => Promise<void>;
  fetchInboxFile: () => Promise<void>;
  setInboxFile: (path: string) => Promise<void>;
  setTodoStateConfig: (config: { todo: string; done: string }) => void;
}

export const useSettingsSlice = create<SettingsState>((set) => ({
  watchedFolders: [],
  inboxFile: null,
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
  fetchInboxFile: async () => {
    const inboxFile = await getInboxFile();
    set({ inboxFile });
  },
  setInboxFile: async (path) => {
    await apiSetInboxFile(path);
    set({ inboxFile: path });
  },
  setTodoStateConfig: (config) => set({ todoStateConfig: config }),
}));
