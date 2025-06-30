## Relevant Files

- `src/components/layout/MainLayout.tsx` - The main layout component that will be updated to a 3-pane view.
- `src/components/FileExplorerPane.tsx` - A new component for browsing files.
- `src/components/TaskListPane.tsx` - The existing component for displaying and editing tasks from a file.
- `src/components/AgendaPane.tsx` - The existing component for the agenda, to be updated.
- `src/App.tsx` - Where the main layout and its panes are assembled.
- `src/index.css` - For global styles and dark theme variables.

### Notes

- Unit tests should be created alongside the components.
- Use `pnpm test` to run tests.

## Tasks

- [x] 1.0 Implement the 3-pane resizable layout in `MainLayout.tsx`
  - [x] 1.1 Update `MainLayout.tsx` to use a 3-panel `PanelGroup` for the file explorer, task list, and agenda panes.
  - [x] 1.2 Add a second `PanelResizeHandle` between the task list and agenda panes.
  - [x] 1.3 Update the Zustand store (in `uiSlice.ts`) to manage three pane sizes.
  - [x] 1.4 Add a `fileExplorer` prop to `MainLayout` and pass it to the first `Panel`.
- [x] 2.0 Create the `FileExplorerPane.tsx` component for file navigation
  - [x] 2.1 Create the new file `src/components/FileExplorerPane.tsx`.
  - [x] 2.2 Create a Tauri command to recursively read files and folders from the `data` directory.
  - [x] 2.3 Display the files and folders in a collapsible tree view structure.
  - [x] 2.4 Add a click handler to files that updates a new `selectedFile` state in the Zustand store.
  - [x] 2.5 Style the component with hover effects and icons, inspired by the Obsidian screenshot.
- [x] 3.0 Update `TaskListPane.tsx` to display content from a selected file
  - [x] 3.1 Modify `TaskListPane.tsx` to subscribe to the `selectedFile` state from the store.
  - [x] 3.2 When `selectedFile` changes, invoke a new Tauri command to fetch the raw content of the file.
  - [x] 3.3 Integrate a parser (like `orgajs`) to transform the Org-mode text into a structured format (AST).
  - [x] 3.4 Render the AST, displaying headings, paragraphs, and interactive task items.
- [x] 4.0 Enhance `AgendaPane.tsx` with agenda and secondary note views
  - [x] 4.1 Refactor the agenda view to parse tasks from all `.org` files within the `data` directory.
  - [x] 4.2 Group tasks by date, showing each date as a distinct heading.
  - [x] 4.3 Add a header to the pane with toggle buttons for "Agenda" and "Note" views.
  - [x] 4.4 Implement the "Note" view, allowing a user to select and render a second file for split-screen viewing.
- [x] 5.0 Apply global styling for a dark theme inspired by Obsidian
  - [x] 5.1 Define dark theme color variables in `src/index.css` for background, text, and accent colors.
  - [x] 5.2 Apply the `dark` class to the root element in `index.html`.
  - [x] 5.3 Update `tailwind.config.js` to use the new CSS variables.
  - [x] 5.4 Style the panel resize handles to be thin and unobtrusive, with a clear hover state.
  - [x] 5.5 Ensure all components adhere to the new dark theme for a consistent, modern UI. 