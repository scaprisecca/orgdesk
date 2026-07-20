import { describe, it, expect } from 'vitest';
import { buildTaskTree, findTask, mapTaskRecursively, removeTaskRecursively, filterTasks } from './taskTree';
import type { Task } from './taskTree';

const makeTask = (overrides: Partial<Task> & Pick<Task, 'id' | 'title' | 'level' | 'filePath'>): Task => ({
  state: 'TODO',
  tags: [],
  properties: {},
  ...overrides,
});

describe('buildTaskTree', () => {
  it('nests children under their parent based on level', () => {
    const flat: Task[] = [
      makeTask({ id: '1', title: 'Parent', level: 1, filePath: 'a.org' }),
      makeTask({ id: '2', title: 'Child', level: 2, filePath: 'a.org' }),
      makeTask({ id: '3', title: 'Grandchild', level: 3, filePath: 'a.org' }),
    ];

    const tree = buildTaskTree(flat);

    expect(tree).toHaveLength(1);
    expect(tree[0].id).toBe('1');
    expect(tree[0].children).toHaveLength(1);
    expect(tree[0].children?.[0].id).toBe('2');
    expect(tree[0].children?.[0].children).toHaveLength(1);
    expect(tree[0].children?.[0].children?.[0].id).toBe('3');
  });

  it('pops back to the right ancestor when level decreases', () => {
    const flat: Task[] = [
      makeTask({ id: '1', title: 'Root A', level: 1, filePath: 'a.org' }),
      makeTask({ id: '2', title: 'Child of A', level: 2, filePath: 'a.org' }),
      makeTask({ id: '3', title: 'Root B', level: 1, filePath: 'a.org' }),
    ];

    const tree = buildTaskTree(flat);

    expect(tree).toHaveLength(2);
    expect(tree[0].children).toHaveLength(1);
    expect(tree[1].children).toHaveLength(0);
  });

  it('keeps tasks from different files in separate outlines even if they interleave', () => {
    const flat: Task[] = [
      makeTask({ id: '1', title: 'File A root', level: 1, filePath: 'a.org' }),
      makeTask({ id: '2', title: 'File B root', level: 1, filePath: 'b.org' }),
      makeTask({ id: '3', title: 'File A child', level: 2, filePath: 'a.org' }),
    ];

    const tree = buildTaskTree(flat);

    // Two independent roots (one per file) — "File A child" must nest under
    // "File A root", not accidentally under "File B root" just because it
    // came right after it in the flat list.
    expect(tree).toHaveLength(2);
    const fileARoot = tree.find((t) => t.id === '1');
    expect(fileARoot?.children).toHaveLength(1);
    expect(fileARoot?.children?.[0].id).toBe('3');
    const fileBRoot = tree.find((t) => t.id === '2');
    expect(fileBRoot?.children).toHaveLength(0);
  });
});

describe('findTask', () => {
  it('finds a top-level task', () => {
    const tasks = [makeTask({ id: '1', title: 'Root', level: 1, filePath: 'a.org' })];
    expect(findTask(tasks, '1')?.title).toBe('Root');
  });

  it('finds a nested task', () => {
    const tasks = [
      {
        ...makeTask({ id: '1', title: 'Root', level: 1, filePath: 'a.org' }),
        children: [makeTask({ id: '2', title: 'Child', level: 2, filePath: 'a.org' })],
      },
    ];
    expect(findTask(tasks, '2')?.title).toBe('Child');
  });

  it('returns undefined when the id does not exist', () => {
    const tasks = [makeTask({ id: '1', title: 'Root', level: 1, filePath: 'a.org' })];
    expect(findTask(tasks, 'missing')).toBeUndefined();
  });
});

describe('mapTaskRecursively', () => {
  it('updates only the matching task, at any depth, leaving siblings untouched', () => {
    const tasks = [
      {
        ...makeTask({ id: '1', title: 'Root', level: 1, filePath: 'a.org' }),
        children: [
          makeTask({ id: '2', title: 'Child A', level: 2, filePath: 'a.org' }),
          makeTask({ id: '3', title: 'Child B', level: 2, filePath: 'a.org' }),
        ],
      },
    ];

    const updated = mapTaskRecursively(tasks, '3', (t) => ({ ...t, title: 'Renamed' }));

    expect(updated[0].children?.[0].title).toBe('Child A');
    expect(updated[0].children?.[1].title).toBe('Renamed');
  });
});

describe('removeTaskRecursively', () => {
  it('removes a nested task without touching its siblings', () => {
    const tasks = [
      {
        ...makeTask({ id: '1', title: 'Root', level: 1, filePath: 'a.org' }),
        children: [
          makeTask({ id: '2', title: 'Child A', level: 2, filePath: 'a.org' }),
          makeTask({ id: '3', title: 'Child B', level: 2, filePath: 'a.org' }),
        ],
      },
    ];

    const result = removeTaskRecursively(tasks, '2');

    expect(result[0].children).toHaveLength(1);
    expect(result[0].children?.[0].id).toBe('3');
  });

  it('removes a top-level task', () => {
    const tasks = [
      makeTask({ id: '1', title: 'Root A', level: 1, filePath: 'a.org' }),
      makeTask({ id: '2', title: 'Root B', level: 1, filePath: 'a.org' }),
    ];

    const result = removeTaskRecursively(tasks, '1');

    expect(result).toHaveLength(1);
    expect(result[0].id).toBe('2');
  });
});

describe('filterTasks', () => {
  it('returns every task when the filter is empty', () => {
    const tasks = [makeTask({ id: '1', title: 'Buy milk', level: 1, filePath: 'a.org' })];
    expect(filterTasks(tasks, '')).toEqual(tasks);
  });

  it('matches case-insensitively on title', () => {
    const tasks = [makeTask({ id: '1', title: 'Buy Milk', level: 1, filePath: 'a.org' })];
    expect(filterTasks(tasks, 'milk')).toHaveLength(1);
    expect(filterTasks(tasks, 'bread')).toHaveLength(0);
  });

  it('keeps a parent whose child matches even if the parent title does not', () => {
    const tasks = [
      {
        ...makeTask({ id: '1', title: 'Groceries', level: 1, filePath: 'a.org' }),
        children: [makeTask({ id: '2', title: 'Buy milk', level: 2, filePath: 'a.org' })],
      },
    ];

    const result = filterTasks(tasks, 'milk');

    expect(result).toHaveLength(1);
    expect(result[0].children).toHaveLength(1);
  });

  it('drops a parent whose children all fail to match', () => {
    const tasks = [
      {
        ...makeTask({ id: '1', title: 'Groceries', level: 1, filePath: 'a.org' }),
        children: [makeTask({ id: '2', title: 'Buy milk', level: 2, filePath: 'a.org' })],
      },
    ];

    expect(filterTasks(tasks, 'bread')).toHaveLength(0);
  });
});
