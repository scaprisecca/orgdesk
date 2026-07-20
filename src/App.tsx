import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { MainLayout } from './components/layout/MainLayout';
import { Toolbar } from './components/Toolbar';
import { TaskListPane } from './components/TaskListPane';
import { AgendaPane } from './components/AgendaPane';
import { QuickCaptureModal } from './components/modals/QuickCaptureModal';
import { RefileDialog } from './components/dialogs/RefileDialog';
import { AgendaBuilderDialog } from './components/dialogs/AgendaBuilderDialog';
import { SettingsDialog } from './components/dialogs/SettingsDialog';
import { useUiSlice, useTasksSlice, useSettingsSlice } from './stores';

function App() {
  const { activeModal, closeModal } = useUiSlice();
  const fetchTasks = useTasksSlice((state) => state.fetchTasks);
  const taskError = useTasksSlice((state) => state.error);
  const fetchWatchedFolders = useSettingsSlice((state) => state.fetchWatchedFolders);
  const fetchInboxFile = useSettingsSlice((state) => state.fetchInboxFile);

  useEffect(() => {
    fetchTasks();
    fetchWatchedFolders();
    fetchInboxFile();
  }, [fetchTasks, fetchWatchedFolders, fetchInboxFile]);

  useEffect(() => {
    const unlisten = listen('tasks-changed', () => {
      fetchTasks();
    });
    return () => {
      unlisten.then((stopListening) => stopListening());
    };
  }, [fetchTasks]);

  return (
    <>
      {taskError && (
        <div className="fixed top-0 inset-x-0 z-50 bg-red-600 text-white text-sm px-4 py-2 flex justify-between items-center">
          <span>{taskError}</span>
          <button onClick={() => useTasksSlice.setState({ error: null })} className="font-bold px-2">
            ×
          </button>
        </div>
      )}
      <MainLayout
        toolbar={<Toolbar />}
        taskList={<TaskListPane />}
        agenda={<AgendaPane />}
      />
      <QuickCaptureModal
        isOpen={activeModal === 'QuickCapture'}
        onClose={closeModal}
      />
      <RefileDialog
        isOpen={activeModal === 'Refile'}
        onClose={closeModal}
      />
      <AgendaBuilderDialog
        isOpen={activeModal === 'AgendaBuilder'}
        onClose={closeModal}
      />
      <SettingsDialog
        isOpen={activeModal === 'Settings'}
        onClose={closeModal}
      />
    </>
  );
}

export default App;
