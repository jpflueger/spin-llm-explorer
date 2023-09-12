const baseUrl = import.meta.env.VITE_API_HOST || '';

async function fetchApps(): Promise<App[]> {
  const response = await fetch(`${baseUrl}/api/apps`);
  return await response.json();
}

async function fetchApp(name: string): Promise<App> {
  const response = await fetch(`${baseUrl}/api/apps/${name}`);
  return await response.json();
}

async function createApp(app: App): Promise<App> {
  const response = await fetch(`${baseUrl}/api/apps`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(app),
  });
  return await response.json();
}

async function updateApp(app: App): Promise<App> {
  const response = await fetch(`${baseUrl}/api/apps/${app.name}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(app),
  });
  return await response.json();
}

async function deleteApp(name: string): Promise<void> {
  await fetch(`${baseUrl}/api/apps/${name}`, {
    method: 'DELETE',
  });
}

export default {
  fetchApps,
  fetchApp,
  createApp,
  updateApp,
  deleteApp,
};
