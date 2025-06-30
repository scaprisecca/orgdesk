import { invoke } from '@tauri-apps/api/core';
import type { Task } from '../stores';

/**
 * This file will contain all the wrapper functions for invoking Rust commands
 * in the Tauri backend. This provides a clean, type-safe way to communicate
 * with the core application logic.
 */

export async function getTasks(): Promise<Task[]> {
  try {
    // Assuming the backend returns data that matches the Task[] structure
    const tasks = await invoke<Task[]>("list_tasks");
    return tasks;
  } catch (error) {
    console.error("Error invoking list_tasks:", error);
    // Return mock data on error for development purposes
    return [
      { id: 'mock1', title: 'Mock task from error', state: 'TODO' }
    ];
  }
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

// TODO: Define other IPC functions as needed, e.g.:
// - getTasks()
// - createTask(taskData)
// - updateTask(taskId, taskData)
// - deleteTask(taskId)
// - getSettings()
// - saveSettings(settings) 