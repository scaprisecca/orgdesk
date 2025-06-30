import { useState, useEffect } from 'react';
import { MainLayout } from './components/layout/MainLayout';
import { Toolbar } from './components/Toolbar';
import { TaskListPane } from './components/TaskListPane';
import { AgendaPane } from './components/AgendaPane';
import { FileExplorerPane } from './components/FileExplorerPane';
import { QuickCaptureModal } from './components/modals/QuickCaptureModal';
import { RefileDialog } from './components/dialogs/RefileDialog';
import { AgendaBuilderDialog } from './components/dialogs/AgendaBuilderDialog';
import { SettingsDialog } from './components/dialogs/SettingsDialog';
import { UpdatePrompt } from './components/ui/UpdatePrompt';
import { useBoundStore } from './stores';
import { updateWatchedFolders } from './lib/api';

function App() {
  const { activeModal, closeModal, watchedFolders, fetchTasks } = useBoundStore();
  const [showUpdatePrompt, setShowUpdatePrompt] = useState(true);

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  useEffect(() => {
    updateWatchedFolders(watchedFolders);
  }, [watchedFolders]);

  return (
    <>
      <MainLayout
        toolbar={<Toolbar />}
        fileExplorer={<FileExplorerPane />}
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
      {showUpdatePrompt && <UpdatePrompt onDismiss={() => setShowUpdatePrompt(false)} />}
    </>
  );
}

export default App;
