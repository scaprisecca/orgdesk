import React, { useState, useEffect } from 'react';
import { MainLayout } from './components/layout/MainLayout';
import { Toolbar } from './components/Toolbar';
import { TaskListPane } from './components/TaskListPane';
import { AgendaPane } from './components/AgendaPane';
import { QuickCaptureModal } from './components/modals/QuickCaptureModal';
import { RefileDialog } from './components/dialogs/RefileDialog';
import { AgendaBuilderDialog } from './components/dialogs/AgendaBuilderDialog';
import { SettingsDialog } from './components/dialogs/SettingsDialog';
import { UpdatePrompt } from './components/ui/UpdatePrompt';
import { useUiSlice, useTasksSlice } from './stores';

function App() {
  const { activeModal, closeModal } = useUiSlice();
  const [showUpdatePrompt, setShowUpdatePrompt] = useState(true);
  const fetchTasks = useTasksSlice((state: any) => state.fetchTasks);

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  return (
    <>
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
      {showUpdatePrompt && <UpdatePrompt onDismiss={() => setShowUpdatePrompt(false)} />}
    </>
  );
}

export default App;
