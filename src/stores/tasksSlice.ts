import { create } from 'zustand';
import { getTasks } from '../lib/api';

export type TodoState = 'TODO' | 'DONE' | 'IN_PROGRESS' | 'SOMEDAY' | 'CANCELED';

export interface Task {
  id: string;
  title: string;
  state: TodoState;
  level: number;
  tags: string[];
  priority?: 'A' | 'B' | 'C';
  scheduled?: string; // YYYY-MM-DD
  deadline?: string; // YYYY-MM-DD
  filePath: string;
  children?: Task[];
}

interface TasksState {
  tasks: Task[];
  fetchTasks: () => Promise<void>;
  addTask: (task: Task) => void;
  updateTaskTitle: (id: string, newTitle: string) => void;
  toggleTaskState: (id: string) => void;
}

// The backend sends a flat list (order preserved per source file, but file
// order itself is not guaranteed) with each task's outline `level`. Bucket
// by file first so cross-file interleaving can't scramble parent/child
// matching, then rebuild each file's outline from levels via a stack.
const buildTaskTree = (flatTasks: Task[]): Task[] => {
  const byFile = new Map<string, Task[]>();
  for (const task of flatTasks) {
    const fileTasks = byFile.get(task.filePath);
    if (fileTasks) {
      fileTasks.push(task);
    } else {
      byFile.set(task.filePath, [task]);
    }
  }

  const roots: Task[] = [];
  for (const fileTasks of byFile.values()) {
    const stack: Task[] = [];
    for (const task of fileTasks) {
      const node: Task = { ...task, children: [] };
      while (stack.length > 0 && stack[stack.length - 1].level >= node.level) {
        stack.pop();
      }
      const parent = stack[stack.length - 1];
      if (parent) {
        parent.children = [...(parent.children ?? []), node];
      } else {
        roots.push(node);
      }
      stack.push(node);
    }
  }
  return roots;
};

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
    set({ tasks: buildTaskTree(tasks) });
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