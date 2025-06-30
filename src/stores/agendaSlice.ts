import type { StateCreator } from 'zustand';
import type { TasksSlice } from './tasksSlice';
import type { SettingsSlice } from './settingsSlice';
import type { UiSlice } from './uiSlice';

export interface AgendaPreset {
  id: string;
  name: string;
  // Define other preset properties here, e.g., date ranges, filters
}

export interface AgendaSlice {
  selectedDate: Date;
  presets: AgendaPreset[];
  activePresetId: string | null;
  setDate: (date: Date) => void;
  addPreset: (preset: AgendaPreset) => void;
  setActivePreset: (id: string) => void;
}

export const createAgendaSlice: StateCreator<
  AgendaSlice & TasksSlice & SettingsSlice & UiSlice,
  [],
  [],
  AgendaSlice
> = (set) => ({
  selectedDate: new Date(),
  presets: [
    { id: '1', name: 'Default' },
    { id: '2', name: 'Work' },
    { id: '3', name: 'Personal' },
  ],
  activePresetId: '1',
  setDate: (date) => set({ selectedDate: date }),
  addPreset: (preset) =>
    set((state) => ({ presets: [...state.presets, preset] })),
  setActivePreset: (id) => set({ activePresetId: id }),
}); 