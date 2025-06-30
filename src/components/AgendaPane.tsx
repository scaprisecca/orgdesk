import React from 'react';
import { useTasksSlice } from '../stores/tasksSlice';
import { useAgendaSlice } from '../stores/agendaSlice';
import type { Task } from '../stores/tasksSlice';
import { Dropdown, DropdownItem } from './ui/Dropdown';
import { ChevronDown } from 'lucide-react';

const formatDate = (date: Date) => {
  return date.toISOString().split('T')[0]; // YYYY-MM-DD
};

const getWeekDays = (startDate: Date): Date[] => {
  const days = [];
  for (let i = 0; i < 7; i++) {
    const day = new Date(startDate);
    day.setDate(day.getDate() + i);
    days.push(day);
  }
  return days;
};

export const AgendaPane = () => {
  const allTasks = useTasksSlice((state) => state.tasks);
  const toggleTaskState = useTasksSlice((state) => state.toggleTaskState);
  const { selectedDate, setDate, presets, activePresetId, setActivePreset } = useAgendaSlice();

  const week = getWeekDays(new Date()); // Start the week from today
  const selectedDateString = formatDate(selectedDate);

  const flattenTasks = (tasks: Task[]): Task[] => {
    let flat: Task[] = [];
    tasks.forEach(task => {
      flat.push(task);
      if (task.children) {
        flat = flat.concat(flattenTasks(task.children));
      }
    });
    return flat;
  };

  const agendaItems = flattenTasks(allTasks).filter(task => {
    return task.scheduled === selectedDateString || task.deadline === selectedDateString;
  });

  return (
    <div className="h-full p-4 bg-gray-50 dark:bg-gray-800 flex flex-col">
      <div className="mb-4">
        <div className="flex justify-between items-center">
          <h2 className="text-xl font-bold">Agenda</h2>
          <Dropdown
            trigger={
              <button className="flex items-center gap-1 text-sm p-1.5 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700">
                <span>{presets.find(p => p.id === activePresetId)?.name || 'Select Preset'}</span>
                <ChevronDown size={16} />
              </button>
            }
          >
            {presets.map(preset => (
              <DropdownItem key={preset.id} onSelect={() => setActivePreset(preset.id)}>
                {preset.name}
              </DropdownItem>
            ))}
          </Dropdown>
        </div>
        <div className="flex gap-1 mt-2">
          {week.map(day => {
            const dayString = formatDate(day);
            const isSelected = dayString === selectedDateString;
            return (
              <button
                key={dayString}
                onClick={() => setDate(day)}
                className={`p-2 text-center rounded-lg flex-1 ${
                  isSelected
                    ? 'bg-blue-500 text-white'
                    : 'bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600'
                }`}
              >
                <div className="font-bold text-sm">{day.toLocaleDateString(undefined, { weekday: 'short' })}</div>
                <div className="text-xs">{day.toLocaleDateString(undefined, { day: 'numeric', month: 'short' })}</div>
              </button>
            );
          })}
        </div>
      </div>
      <div className="space-y-4 flex-grow overflow-y-auto">
        {agendaItems.length > 0 ? (
          agendaItems.map((item) => (
            <div key={item.id} className="flex items-center gap-4">
              <input
                type="checkbox"
                checked={item.state === 'DONE'}
                onChange={() => toggleTaskState(item.id)}
                className="h-4 w-4 rounded"
              />
              <div className={`flex-grow p-2 bg-white dark:bg-gray-700 rounded-lg shadow ${item.state === 'DONE' ? 'line-through text-gray-500' : ''}`}>
                {item.title}
                <div className="text-xs text-gray-400">
                  {item.deadline ? `Deadline` : `Scheduled`}
                </div>
              </div>
            </div>
          ))
        ) : (
          <p className="text-gray-500">No items on the agenda for {selectedDate.toLocaleDateString(undefined, { weekday: 'long' })}.</p>
        )}
      </div>
    </div>
  );
};
