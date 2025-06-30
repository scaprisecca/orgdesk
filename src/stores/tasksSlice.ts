import { create } from 'zustand';
import { getTasks } from '../lib/api';

export interface Task {
  id: string;
  title: string;
  state: 'TODO' | 'DONE';
  children?: Task[];
  scheduled?: string; // YYYY-MM-DD
  deadline?: string; // YYYY-MM-DD
}

interface TasksState {
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

export const useTasksSlice = create<TasksState>((set) => ({
  tasks: [],
  fetchTasks: async () => {
    const tasks = await getTasks();
    set({ tasks });
  },
  addTask: (task) => set((state) => ({ tasks: [...state.tasks, task] })),
  updateTaskTitle: (id, newTitle) =>
    set((state) => ({
      tasks: updateTaskRecursively(state.tasks, id, newTitle),
    })),
  toggleTaskState: (id) =>
    set((state) => ({
      tasks: toggleTaskStateRecursively(state.tasks, id),
    })),
})); 