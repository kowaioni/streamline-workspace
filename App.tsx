import React, { useState, useEffect } from 'react';

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

const mockProjects: Project[] = [
];

const mockUsers: User[] = [
];

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
            <li key={project.id} onClick={() => handleProjectSelect(project.id)}>
              {project.name}
            </li>
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
                <li key={task.id}>
                  <h4>{task.title}</h4>
                  <p>{task.description}</p>
                  <p>Assigned to: {task.assignedTo.name}</p>
                  <p>Status: {task.status}</p>
                </li>
              ))}
            </ul>
          </>
        )}
      </div>
      
      <div className="user-profile">
        <h2>Users</h2>
        <ul>
          {users.map((user) => (
            <li key={user.id}>
              <h3>{user.name}</h3>
              <p>{user.email}</p>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default StreamlineWorkspace;