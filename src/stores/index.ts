import { create } from 'zustand';
import { createTasksSlice, type TasksSlice } from './tasksSlice';
import { createSettingsSlice, type SettingsSlice } from './settingsSlice';
import { createAgendaSlice, type AgendaSlice } from './agendaSlice';
import { createUiSlice, type UiSlice } from './uiSlice';

export const useBoundStore = create<TasksSlice & SettingsSlice & AgendaSlice & UiSlice>()((...a) => ({
  ...createTasksSlice(...a),
  ...createSettingsSlice(...a),
  ...createAgendaSlice(...a),
  ...createUiSlice(...a),
}));

export * from './tasksSlice';
export * from './settingsSlice';
export * from './agendaSlice';
export * from './uiSlice'; 