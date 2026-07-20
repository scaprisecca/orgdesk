import { create } from 'zustand';
import { getTasks, createTask, updateTask, deleteTask } from '../lib/api';

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
  properties: Record<string, string>;
  filePath: string;
  children?: Task[];
}

interface TasksState {
  tasks: Task[];
  error: string | null;
  fetchTasks: () => Promise<void>;
  addTask: (title: string) => Promise<void>;
  updateTaskTitle: (id: string, newTitle: string) => Promise<void>;
  toggleTaskState: (id: string) => Promise<void>;
  removeTask: (id: string) => Promise<void>;
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

const findTask = (tasks: Task[], id: string): Task | undefined => {
  for (const task of tasks) {
    if (task.id === id) {
      return task;
    }
    if (task.children) {
      const found = findTask(task.children, id);
      if (found) {
        return found;
      }
    }
  }
  return undefined;
};

const mapTaskRecursively = (tasks: Task[], id: string, updater: (task: Task) => Task): Task[] => {
  return tasks.map(task => {
    if (task.id === id) {
      return updater(task);
    }
    if (task.children) {
      return { ...task, children: mapTaskRecursively(task.children, id, updater) };
    }
    return task;
  });
};

const removeTaskRecursively = (tasks: Task[], id: string): Task[] => {
  return tasks
    .filter(task => task.id !== id)
    .map(task => (task.children ? { ...task, children: removeTaskRecursively(task.children, id) } : task));
};

const errorMessage = (error: unknown, fallback: string): string =>
  error instanceof Error ? error.message : fallback;

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
      await updateTask({ ...task, title: newTitle });
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
      await updateTask({ ...task, state: newState });
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
