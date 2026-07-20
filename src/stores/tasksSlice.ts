import { create } from 'zustand';
import { getTasks, createTask, updateTask, deleteTask, isBackendError } from '../lib/api';
import { buildTaskTree, findTask, mapTaskRecursively, removeTaskRecursively } from '../lib/taskTree';
import type { Task, TodoState } from '../lib/taskTree';

export type { Task, TodoState };

interface TasksState {
  tasks: Task[];
  error: string | null;
  fetchTasks: () => Promise<void>;
  addTask: (title: string) => Promise<void>;
  updateTaskTitle: (id: string, newTitle: string) => Promise<void>;
  toggleTaskState: (id: string) => Promise<void>;
  removeTask: (id: string) => Promise<void>;
}

const errorMessage = (error: unknown, fallback: string): string => {
  if (error instanceof Error) return error.message;
  if (isBackendError(error)) return error.message;
  return fallback;
};

export const useTasksSlice = create<TasksState>((set, get) => ({
  tasks: [],
  error: null,
  fetchTasks: async () => {
    try {
      const tasks = await getTasks();
      set({ tasks: buildTaskTree(tasks), error: null });
    } catch (error) {
      set({ error: errorMessage(error, 'Failed to load tasks') });
    }
  },
  addTask: async (title) => {
    try {
      await createTask(title);
      await get().fetchTasks();
    } catch (error) {
      set({ error: errorMessage(error, 'Failed to create task') });
    }
  },
  updateTaskTitle: async (id, newTitle) => {
    const task = findTask(get().tasks, id);
    if (!task) return;
    const previousTasks = get().tasks;

    set({
      tasks: mapTaskRecursively(previousTasks, id, (t) => ({ ...t, title: newTitle })),
      error: null,
    });
    try {
      await updateTask(id, { title: newTitle });
    } catch (error) {
      set({ tasks: previousTasks, error: errorMessage(error, 'Failed to update task') });
    }
  },
  toggleTaskState: async (id) => {
    const task = findTask(get().tasks, id);
    if (!task) return;
    const previousTasks = get().tasks;
    const newState: TodoState = task.state === 'DONE' ? 'TODO' : 'DONE';

    set({
      tasks: mapTaskRecursively(previousTasks, id, (t) => ({ ...t, state: newState })),
      error: null,
    });
    try {
      await updateTask(id, { state: newState });
    } catch (error) {
      set({ tasks: previousTasks, error: errorMessage(error, 'Failed to update task') });
    }
  },
  removeTask: async (id) => {
    const previousTasks = get().tasks;
    set({ tasks: removeTaskRecursively(previousTasks, id), error: null });
    try {
      await deleteTask(id);
    } catch (error) {
      set({ tasks: previousTasks, error: errorMessage(error, 'Failed to delete task') });
    }
  },
}));
