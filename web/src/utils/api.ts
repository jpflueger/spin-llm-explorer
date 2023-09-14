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

function createCompletion(app: App, prompt: string): Promise<Completion> {
  return fetch(`${baseUrl}/api/completions`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      messages: [{
        role: 'system',
        content: app.system_prompt,
      }, {
        role: 'user',
        content: prompt,
        }],
      model: app.model.name,
      params: {
        max_tokens: app.model.max_tokens,
        temperature: app.model.temperature,
        repeat_penalty: app.model.repeat_penalty,
        repeat_penalty_last_n_tokens: app.model.repeat_penalty_last_n_tokens,
        top_k: app.model.top_k,
        top_p: app.model.top_p,
      },
     }),
  }).then(response => response.json());
}

export default {
  fetchApps,
  fetchApp,
  createApp,
  updateApp,
  deleteApp,
  createCompletion,
};
