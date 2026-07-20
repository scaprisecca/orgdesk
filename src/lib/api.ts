import { invoke } from '@tauri-apps/api/core';
import type { Task } from '../stores';

/**
 * This file will contain all the wrapper functions for invoking Rust commands
 * in the Tauri backend. This provides a clean, type-safe way to communicate
 * with the core application logic.
 */

export async function getTasks(): Promise<Task[]> {
  return invoke<Task[]>("list_tasks");
}

export async function createTask(title: string): Promise<Task> {
  return invoke<Task>('create_task', { title });
}

export async function updateTask(task: Task): Promise<Task> {
  return invoke<Task>('update_task', { task });
}

export async function deleteTask(taskId: string): Promise<Task> {
  return invoke<Task>('delete_task', { taskId });
}

export async function getAgendaRange(startDate: string, endDate: string): Promise<Task[]> {
  return invoke<Task[]>('get_agenda_range', { startDate, endDate });
}

export async function getInboxFile(): Promise<string | null> {
  return invoke<string | null>('get_inbox_file');
}

export async function setInboxFile(path: string): Promise<void> {
  return invoke<void>('set_inbox_file', { path });
}

// Example function for parsing org content
export async function parseOrgContent(content: string): Promise<any> {
  try {
    const result = await invoke("parse_org_content", { content });
    return result;
  } catch (error) {
    console.error("Error invoking parse_org_content:", error);
    throw error;
  }
}

export async function addWatchedFolder(path: string): Promise<string[]> {
  return invoke<string[]>('add_watched_folder', { path });
}

export async function removeWatchedFolder(path: string): Promise<string[]> {
  return invoke<string[]>('remove_watched_folder', { path });
}

export async function getWatchedFolders(): Promise<string[]> {
  return invoke<string[]>('get_watched_folders');
} 