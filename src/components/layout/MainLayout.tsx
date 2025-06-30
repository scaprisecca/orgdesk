import React from 'react';
import {
  Panel,
  PanelGroup,
  PanelResizeHandle,
} from 'react-resizable-panels';
import { useUiSlice } from '../../stores';

export const MainLayout = ({
  toolbar,
  taskList,
  agenda,
}: {
  toolbar: React.ReactNode;
  taskList: React.ReactNode;
  agenda: React.ReactNode;
}) => {
  const { paneSizes, setPaneSizes } = useUiSlice();

  return (
    <div className="flex flex-col h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
      <header className="flex-shrink-0">{toolbar}</header>
      <main className="flex-grow">
        <PanelGroup
          direction="horizontal"
          onLayout={(sizes: number[]) => setPaneSizes(sizes)}
          className="h-full"
        >
          <Panel defaultSize={paneSizes[0]} minSize={20}>
            {taskList}
          </Panel>
          <PanelResizeHandle className="w-1 bg-gray-200 dark:bg-gray-700 hover:bg-blue-500 transition-colors" />
          <Panel defaultSize={paneSizes[1]} minSize={30}>
            {agenda}
          </Panel>
        </PanelGroup>
      </main>
    </div>
  );
}; 