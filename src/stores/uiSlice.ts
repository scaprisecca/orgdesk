import { create } from 'zustand';
import { persist } from 'zustand/middleware';

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

// `paneSizes` is persisted (via `partialize`, below) so `MainLayout`'s
// `defaultSize={paneSizes[...]}` — read only once, on mount — reflects the
// user's last resize instead of always resetting to [30, 70] on every
// restart (see L5 in the code review).
export const useUiSlice = create<UiState>()(
  persist(
    (set) => ({
      activeModal: null,
      isVimMode: false,
      paneSizes: [30, 70], // Example initial sizes
      openModal: (modal) => set({ activeModal: modal }),
      closeModal: () => set({ activeModal: null }),
      toggleVimMode: () => set((state) => ({ isVimMode: !state.isVimMode })),
      setPaneSizes: (sizes) => set({ paneSizes: sizes }),
    }),
    {
      name: 'orgdesk-ui',
      partialize: (state) => ({ paneSizes: state.paneSizes }),
    },
  ),
); 