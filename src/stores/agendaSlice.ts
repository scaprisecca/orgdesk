import { create } from 'zustand';

interface AgendaPreset {
  id: string;
  name: string;
  // Define other preset properties here, e.g., date ranges, filters
}

interface AgendaState {
  selectedDate: Date;
  presets: AgendaPreset[];
  activePresetId: string | null;
  setDate: (date: Date) => void;
  addPreset: (preset: AgendaPreset) => void;
  setActivePreset: (id: string) => void;
}

export const useAgendaSlice = create<AgendaState>((set) => ({
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
})); 