import React, { useState, useRef, useEffect, useMemo } from 'react';
import { useTasksSlice } from '../stores';
import type { Task } from '../stores/tasksSlice';

const TaskItem = ({ task, level }: { task: Task, level: number }) => {
  const [isEditing, setIsEditing] = useState(false);
  const [title, setTitle] = useState(task.title);
  const { updateTaskTitle, toggleTaskState } = useTasksSlice(
    (state) => ({
      updateTaskTitle: state.updateTaskTitle,
      toggleTaskState: state.toggleTaskState,
    }),
  );
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isEditing) {
      inputRef.current?.focus();
    }
  }, [isEditing]);

  const handleSave = () => {
    if (title.trim() !== task.title) {
      updateTaskTitle(task.id, title.trim());
    }
    setIsEditing(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleSave();
    } else if (e.key === 'Escape') {
      setTitle(task.title);
      setIsEditing(false);
    }
  };

  return (
    <>
      <li
        className={`flex items-center gap-2 p-2 rounded group ${
          task.state === 'DONE' ? 'line-through text-gray-500' : ''
        }`}
        style={{ paddingLeft: `${level * 1.5}rem` }}
      >
        <input
          type="checkbox"
          checked={task.state === 'DONE'}
          onChange={() => toggleTaskState(task.id)}
          className="mr-2"
        />
        {isEditing ? (
          <input
            ref={inputRef}
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            onBlur={handleSave}
            onKeyDown={handleKeyDown}
            className="bg-transparent border-b border-blue-500 outline-none w-full"
          />
        ) : (
          <span onClick={() => setIsEditing(true)} className="flex-grow cursor-pointer">
            {task.title}
          </span>
        )}
      </li>
      {task.children && (
        <ul className="pl-4">
          {task.children.map((child: Task) => (
            <TaskItem key={child.id} task={child} level={level + 1} />
          ))}
        </ul>
      )}
    </>
  );
};

const filterTasks = (tasks: Task[], filterText: string): Task[] => {
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

export const TaskListPane = () => {
  const tasks = useTasksSlice((state) => state.tasks);
  const [filterText, setFilterText] = useState('');

  const filteredTasks = useMemo(() => filterTasks(tasks, filterText), [tasks, filterText]);

  return (
    <div className="h-full p-4 bg-white dark:bg-gray-800 flex flex-col">
      <div className="mb-4">
        <h2 className="text-xl font-bold">Task List</h2>
        <input
          type="text"
          placeholder="Filter tasks..."
          value={filterText}
          onChange={(e) => setFilterText(e.target.value)}
          className="w-full p-2 mt-2 border rounded bg-gray-50 dark:bg-gray-700 dark:border-gray-600"
        />
      </div>
      <ul className="flex-grow overflow-y-auto">
        {filteredTasks.map((task) => (
          <TaskItem key={task.id} task={task} level={0} />
        ))}
      </ul>
    </div>
  );
}; 