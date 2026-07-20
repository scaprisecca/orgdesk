import { useState, useRef, useEffect, useMemo } from 'react';
import type { KeyboardEvent } from 'react';
import { Trash2 } from 'lucide-react';
import { useTasksSlice } from '../stores';
import type { Task } from '../stores/tasksSlice';
import { filterTasks } from '../lib/taskTree';

const TaskItem = ({ task, level }: { task: Task, level: number }) => {
  const [isEditing, setIsEditing] = useState(false);
  const [title, setTitle] = useState(task.title);
  const updateTaskTitle = useTasksSlice((state) => state.updateTaskTitle);
  const toggleTaskState = useTasksSlice((state) => state.toggleTaskState);
  const removeTask = useTasksSlice((state) => state.removeTask);
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

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
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
        <button
          onClick={() => removeTask(task.id)}
          className="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-600"
          aria-label={`Delete ${task.title}`}
        >
          <Trash2 size={16} />
        </button>
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