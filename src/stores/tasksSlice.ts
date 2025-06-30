import type { StateCreator } from 'zustand';
import { getTasks } from '../lib/api';
import type { SettingsSlice } from './settingsSlice';
import type { AgendaSlice } from './agendaSlice';
import type { UiSlice } from './uiSlice';

export interface Task {
  id: string;
  title: string;
  state: 'TODO' | 'DONE';
  children?: Task[];
  scheduled?: string; // YYYY-MM-DD
  deadline?: string; // YYYY-MM-DD
  filePath?: string;
}

export interface TasksSlice {
  tasks: Task[];
  fetchTasks: () => Promise<void>;
  addTask: (task: Task) => void;
  updateTaskTitle: (id: string, newTitle: string) => void;
  toggleTaskState: (id: string) => void;
}

const updateTaskRecursively = (tasks: Task[], id: string, title: string): Task[] => {
  return tasks.map(task => {
    if (task.id === id) {
      return { ...task, title };
    }
    if (task.children) {
      return { ...task, children: updateTaskRecursively(task.children, id, title) };
    }
    return task;
  });
};

const toggleTaskStateRecursively = (tasks: Task[], id: string): Task[] => {
  return tasks.map(task => {
    if (task.id === id) {
      return { ...task, state: task.state === 'TODO' ? 'DONE' : 'TODO' };
    }
    if (task.children) {
      return { ...task, children: toggleTaskStateRecursively(task.children, id) };
    }
    return task;
  });
};

export const createTasksSlice: StateCreator<
  TasksSlice & SettingsSlice & AgendaSlice & UiSlice,
  [],
  [],
  TasksSlice
> = (set, get) => ({
  tasks: [],
  fetchTasks: async () => {
    const watchedFolders = get().watchedFolders;
    const tasks = await getTasks(watchedFolders);
    set({ tasks });
  },
  addTask: (task) => set((state: TasksSlice) => ({ tasks: [...state.tasks, task] })),
  updateTaskTitle: (id, newTitle) =>
    set((state: TasksSlice) => ({
      tasks: updateTaskRecursively(state.tasks, id, newTitle),
    })),
  toggleTaskState: (id) =>
    set((state: TasksSlice) => ({
      tasks: toggleTaskStateRecursively(state.tasks, id),
    })),
}); 