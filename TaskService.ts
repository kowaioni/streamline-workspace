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

class CacheStore<T> {
    private cache: Map<string | number, T>;

    constructor() {
        this.cache = new Map();
    }

    get(key: string | number): T | undefined {
        return this.cache.get(key);
    }

    set(key: string | number, value: T): void {
        this.cache.set(key, value);
    }

    clear(): void {
        this.cache.clear();
    }
}

class ProjectTaskManager {
    private static projectCache = new CacheStore<Project>();
    private static taskCache = new CacheStore<Task>();

    static async fetchProjectById(projectId: number): Promise<Project> {
        const cachedProject = this.projectCache.get(projectId);
        if (cachedProject) {
            return cachedProject;
        }

        const response = await httpClient.get<Project>(`/projects/${projectId}`);
        const project = response.data;

        this.projectCache.set(projectId, project);

        return project;
    }

    static async updateTaskStatusById(taskId: number, newStatus: Task['status']): Promise<Task> {
        const response = await httpClient.patch<Task>(`/tasks/${taskId}`, {status: newStatus});
        const task = response.data;

        const cachedTask = this.taskCache.get(taskId);
        if (cachedTask) {
            this.taskCache.set(taskId, {...cachedTask, status: newStatus});
        } else {
            this.taskCache.set(taskId, task);
        }

        return task;
    }

    static async createTask(newTaskDetails: Omit<Task, 'id'>): Promise<Task> {
        const response = await httpClient.post<Task>('/tasks', newTaskDetails);
        const task = response.data;
        
        return task;
    }
}

export default ProjectTaskManager;