import React, { useState, useEffect, FC } from 'react';

interface Project {
  id: string;
  name: string;
  description: string;
  tasks: Task[];
}

interface Task {
  id: string;
  title: string;
  description: string;
  assignedTo: User;
  status: 'todo' | 'in progress' | 'done';
}

interface User {
  id: string;
  name: string;
  email: string;
}

const mockProjects: Project[] = [];
const mockUsers: User[] = [];

const ProjectItem: FC<{ project: Project; onSelect: (projectId: string) => void }> = ({ project, onSelect }) => (
  <li onClick={() => onSelect(project.id)}>{project.name}</li>
);

const TaskItem: FC<{ task: Task }> = ({ task }) => (
  <li>
    <h4>{task.title}</h4>
    <p>{task.description}</p>
    <p>Assigned to: {task.assignedTo.name}</p>
    <p>Status: {task.status}</p>
  </li>
);

const UserItem: FC<{ user: User }> = ({ user }) => (
  <li>
    <h3>{user.name}</h3>
    <p>{user.email}</p>
  </li>
);

const StreamlineWorkspace: React.FC = () => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [selectedProject, setSelectedProject] = useState<Project | null>(null);
  const [users, setUsers] = useState<User[]>([]);

  useEffect(() => {
    setProjects(mockProjects);
    setUsers(mockUsers);
  }, []);

  const handleProjectSelect = (projectId: string) => {
    const project = projects.find((project) => project.id === projectId);
    setSelectedProject(project || null);
  };

  return (
    <div className="streamline-workspace">
      <header>
        <h1>Streamline Workspace</h1>
      </header>
      
      <div className="dashboard">
        <h2>Projects</h2>
        <ul>
          {projects.map((project) => (
            <ProjectItem key={project.id} project={project} onSelect={handleProjectSelect} />
          ))}
        </ul>
      </div>
      
      <div className="project-details">
        {selectedProject && (
          <>
            <h2>{selectedProject.name}</h2>
            <p>{selectedProject.description}</p>
            <h3>Tasks</h3>
            <ul>
              {selectedProject.tasks.map((task) => (
                <TaskItem key={task.id} task={task} />
              ))}
            </ul>
          </>
        )}
      </div>
      
      <div className="user-profile">
        <h2>Users</h2>
        <ul>
          {users.map((user) => (
            <UserItem key={user.id} user={user} />
          ))}
        </ul>
      </div>
    </div>
  );
};

export default StreamlineWorkspace;