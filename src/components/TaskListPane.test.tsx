import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { TaskListPane } from './TaskListPane';
import { useTasksSlice } from '../stores/tasksSlice';
import type { Task } from '../lib/taskTree';

const makeTask = (overrides: Partial<Task> & Pick<Task, 'id' | 'title' | 'level' | 'filePath'>): Task => ({
  state: 'TODO',
  tags: [],
  properties: {},
  ...overrides,
});

describe('<TaskListPane />', () => {
  beforeEach(() => {
    useTasksSlice.setState({ tasks: [], error: null });
  });

  it('renders tasks from the store and filters them as the user types', async () => {
    useTasksSlice.setState({
      tasks: [
        makeTask({ id: '1', title: 'Buy milk', level: 1, filePath: 'a.org' }),
        makeTask({ id: '2', title: 'Walk the dog', level: 1, filePath: 'a.org' }),
      ],
    });

    const user = userEvent.setup();
    render(<TaskListPane />);

    expect(screen.getByText('Buy milk')).toBeInTheDocument();
    expect(screen.getByText('Walk the dog')).toBeInTheDocument();

    await user.type(screen.getByPlaceholderText('Filter tasks...'), 'milk');

    expect(screen.getByText('Buy milk')).toBeInTheDocument();
    expect(screen.queryByText('Walk the dog')).not.toBeInTheDocument();
  });
});
