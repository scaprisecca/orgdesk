import React from 'react';
import {
  Panel,
  PanelGroup,
  PanelResizeHandle,
} from 'react-resizable-panels';
import { useBoundStore } from '../../stores';

export const MainLayout = ({
  toolbar,
  fileExplorer,
  taskList,
  agenda,
}: {
  toolbar: React.ReactNode;
  fileExplorer: React.ReactNode;
  taskList: React.ReactNode;
  agenda: React.ReactNode;
}) => {
  const { paneSizes, setPaneSizes } = useBoundStore();

  return (
    <div className="flex flex-col h-screen bg-background text-foreground">
      <header className="flex-shrink-0">{toolbar}</header>
      <main className="flex-grow">
        <PanelGroup
          direction="horizontal"
          onLayout={(sizes: number[]) => setPaneSizes(sizes)}
          className="h-full"
        >
          <Panel defaultSize={paneSizes[0]} minSize={15} collapsible>
            {fileExplorer}
          </Panel>
          <PanelResizeHandle className="w-px bg-border hover:bg-accent transition-colors" />
          <Panel defaultSize={paneSizes[1]} minSize={30}>
            {taskList}
          </Panel>
          <PanelResizeHandle className="w-px bg-border hover:bg-accent transition-colors" />
          <Panel defaultSize={paneSizes[2]} minSize={20} collapsible>
            {agenda}
          </Panel>
        </PanelGroup>
      </main>
    </div>
  );
}; 