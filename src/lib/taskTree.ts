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

// The backend sends a flat list (order preserved per source file, but file
// order itself is not guaranteed) with each task's outline `level`. Bucket
// by file first so cross-file interleaving can't scramble parent/child
// matching, then rebuild each file's outline from levels via a stack.
export const buildTaskTree = (flatTasks: Task[]): Task[] => {
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

export const findTask = (tasks: Task[], id: string): Task | undefined => {
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

export const mapTaskRecursively = (tasks: Task[], id: string, updater: (task: Task) => Task): Task[] => {
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

export const removeTaskRecursively = (tasks: Task[], id: string): Task[] => {
  return tasks
    .filter(task => task.id !== id)
    .map(task => (task.children ? { ...task, children: removeTaskRecursively(task.children, id) } : task));
};

export const filterTasks = (tasks: Task[], filterText: string): Task[] => {
  if (!filterText) {
    return tasks;
  }

  const lowercasedFilter = filterText.toLowerCase();

  return tasks.reduce((acc: Task[], task) => {
    const children = task.children ? filterTasks(task.children, filterText) : [];
    if (task.title.toLowerCase().includes(lowercasedFilter) || children.length > 0) {
      acc.push({ ...task, children });
    }
    return acc;
  }, []);
};
