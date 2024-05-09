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

const apiClient = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_KEY}`
    }
});

class ProjectTaskService {
    static async fetchProjectDetails(projectId: number): Promise<Project> {
        const response = await apiClient.get<Project>(`/projects/${projectId}`);
        return response.data;
    }

    static async updateTaskStatus(taskId: number, status: Task['status']): Promise<Task> {
        const response = await apiClient.patch<Task>(`/tasks/${taskId}`, {status});
        return response.data;
    }

    static async addNewTask(task: Omit<Task, 'id'>): Promise<Task> {
        const response = await apiClient.post<Task>('/tasks', task);
        return response.data;
    }
}

export default ProjectTaskService;