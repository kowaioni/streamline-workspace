import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || '';
const API_KEY = process.env.REACT_APP_API_KEY || '';

interface Task {
    id: number;
    projectId: number;
    title: string;
    description: string;
    status: 'pending' | 'in progress' | 'completed';
}

interface Project {
    id: number;
    name: string;
    description: string;
    tasks: Task[];
}

const httpClient = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
    }
});

class ProjectTaskManager {
    static async fetchProjectById(projectId: number): Promise<Project> {
        const response = await httpClient.get<Project>(`/projects/${projectId}`);
        return response.data;
    }

    static async updateTaskStatusById(taskId: number, newStatus: Task['status']): Promise<Task> {
        const response = await httpClient.patch<Task>(`/tasks/${taskId}`, {status: newStatus});
        return response.data;
    }

    static async createTask(newTaskDetails: Omit<Task, 'id'>): Promise<Task> {
        const response = await httpClient.post<Task>('/tasks', newTaskDetails);
        return response.data;
    }
}

export default ProjectTaskManager;