import { create } from 'zustand';

type ModalType = 'QuickCapture' | 'Refile' | 'AgendaBuilder' | 'Settings' | null;

interface UiState {
  activeModal: ModalType;
  isVimMode: boolean;
  paneSizes: number[];
  openModal: (modal: ModalType) => void;
  closeModal: () => void;
  toggleVimMode: () => void;
  setPaneSizes: (sizes: number[]) => void;
}

export const useUiSlice = create<UiState>((set) => ({
  activeModal: null,
  isVimMode: false,
  paneSizes: [30, 70], // Example initial sizes
  openModal: (modal) => set({ activeModal: modal }),
  closeModal: () => set({ activeModal: null }),
  toggleVimMode: () => set((state) => ({ isVimMode: !state.isVimMode })),
  setPaneSizes: (sizes) => set({ paneSizes: sizes }),
})); 