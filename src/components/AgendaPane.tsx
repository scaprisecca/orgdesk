import { useState, useEffect } from 'react';
import { useBoundStore } from '../stores';
import type { Task } from '../stores';
import { Dropdown, DropdownItem } from './ui/Dropdown';
import { ChevronDown } from 'lucide-react';
import { OrgNode } from './OrgNode';
import { invoke } from '@tauri-apps/api/core';
import { parse } from 'org';
import type { Root } from 'org';

const formatDate = (date: Date) => {
  return date.toISOString().split('T')[0]; // YYYY-MM-DD
};

const getWeekDays = (startDate: Date): Date[] => {
  const days = [];
  for (let i = 0; i < 7; i++) {
    const day = new Date(startDate);
    day.setDate(day.getDate() + i);
    days.push(day);
  }
  return days;
};

// New type for grouped agenda items
type AgendaItemGroup = {
  date: string;
  tasks: Task[];
};

const NoteView = () => {
  const { secondarySelectedFile, setSecondarySelectedFile } = useBoundStore();
  const [fileList, setFileList] = useState<any[]>([]);
  const [ast, setAst] = useState<Root | null>(null);
  const [error, setError] = useState('');

  // Fetch all .org files
  useEffect(() => {
    const getAllFiles = async () => {
      try {
        const rootNode = await invoke<any>('read_fs', { dir: 'data' });
        
        const extractFiles = (node: any, fileList: any[]): any[] => {
          if (!node.is_dir && node.name.endsWith('.org')) {
            fileList.push(node);
          }
          if (node.children) {
            for (const child of node.children) {
              extractFiles(child, fileList);
            }
          }
          return fileList;
        }

        const files = extractFiles(rootNode, []);
        setFileList(files);
      } catch (e) {
        setError(String(e));
      }
    };

    getAllFiles();
  }, []);

  // Fetch content of selected file
  useEffect(() => {
    if (!secondarySelectedFile) {
      setAst(null);
      return;
    }
    invoke<string>('get_file_content', { path: secondarySelectedFile }).then((content: string) => {
      setAst(parse(content));
      setError('');
    }).catch((err: any) => {
      setError(String(err));
      setAst(null);
    });
  }, [secondarySelectedFile]);

  return (
    <div className="flex flex-col h-full">
      <div className="flex-shrink-0 p-2 border-b border-border">
        <select
          value={secondarySelectedFile || ''}
          onChange={(e) => setSecondarySelectedFile(e.target.value || null)}
          className="w-full p-2 bg-secondary rounded"
        >
          <option value="">Select a note...</option>
          {fileList.map(file => <option key={file.path} value={file.path}>{file.name}</option>)}
        </select>
      </div>
      <div className="flex-grow overflow-y-auto p-4">
        {error ? <p className="text-destructive-foreground">{error}</p> : ast ? <OrgNode node={ast} /> : <p className="text-muted-foreground">Select a note to display.</p>}
      </div>
    </div>
  )
}

export const AgendaPane = () => {
  const { 
    tasks: allTasks,
    toggleTaskState,
    presets, 
    activePresetId, 
    setActivePreset,
    rightPaneView,
    setRightPaneView,
  } = useBoundStore();

  const week = getWeekDays(new Date()); 

  const flattenTasks = (tasks: Task[]): Task[] => {
    let flat: Task[] = [];
    tasks.forEach(task => {
      flat.push(task);
      if (task.children) {
        flat = flat.concat(flattenTasks(task.children));
      }
    });
    return flat;
  };

  const allFlattenedTasks = flattenTasks(allTasks);

  const agendaItemsByDate: AgendaItemGroup[] = week.map(date => {
    const dateString = formatDate(date);
    const tasksForDay = allFlattenedTasks.filter(task => {
      return task.scheduled === dateString || task.deadline === dateString;
    });
    return { date: dateString, tasks: tasksForDay };
  }).filter(group => group.tasks.length > 0); // Only keep days with tasks

  const ToggleButton = ({ view, currentView, setView, children }: { view: any, currentView: any, setView: any, children: any }) => (
    <button
      onClick={() => setView(view)}
      className={`px-3 py-1 text-sm font-medium rounded-md ${
        currentView === view
          ? 'bg-primary text-primary-foreground'
          : 'bg-secondary hover:bg-secondary/80'
      }`}
    >
      {children}
    </button>
  );

  return (
    <div className="h-full p-4 bg-background/50 flex flex-col min-h-0">
      <div className="mb-4 flex-shrink-0">
        <div className="flex justify-between items-center">
          <div className="flex items-center gap-2">
            <ToggleButton view="Agenda" currentView={rightPaneView} setView={setRightPaneView}>Agenda</ToggleButton>
            <ToggleButton view="Note" currentView={rightPaneView} setView={setRightPaneView}>Note</ToggleButton>
          </div>
          <Dropdown
            trigger={
              <button className="flex items-center gap-1 text-sm p-1.5 border border-gray-300 dark:border-gray-600 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-700">
                <span>{presets.find(p => p.id === activePresetId)?.name || 'Select Preset'}</span>
                <ChevronDown size={16} />
              </button>
            }
          >
            {presets.map(preset => (
              <DropdownItem key={preset.id} onSelect={() => setActivePreset(preset.id)}>
                {preset.name}
              </DropdownItem>
            ))}
          </Dropdown>
        </div>
      </div>
      {rightPaneView === 'Agenda' ? (
        <div className="space-y-4 flex-grow overflow-y-auto pr-2">
          {agendaItemsByDate.length > 0 ? (
            agendaItemsByDate.map((group) => (
              <div key={group.date}>
                <h3 className="font-bold text-lg mb-2 border-b border-border pb-1">
                  {new Date(group.date).toLocaleDateString(undefined, { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' })}
                </h3>
                <div className="space-y-2">
                  {group.tasks.map(item => (
                    <div key={item.id} className="flex items-center gap-4">
                      <input
                        type="checkbox"
                        checked={item.state === 'DONE'}
                        onChange={() => toggleTaskState(item.id)}
                        className="h-4 w-4 rounded"
                      />
                      <div className={`flex-grow p-2 bg-card rounded-lg shadow ${item.state === 'DONE' ? 'line-through text-muted-foreground' : ''}`}>
                        {item.title}
                        <div className="text-xs text-muted-foreground">
                          {item.filePath}
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            ))
          ) : (
            <p className="text-muted-foreground text-center pt-10">No upcoming tasks for the next 7 days.</p>
          )}
        </div>
      ) : (
        <div className="flex-grow flex items-center justify-center">
          <NoteView />
        </div>
      )}
    </div>
  );
};
