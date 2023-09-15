import { ref, computed } from 'vue';
import { defineStore } from 'pinia';
import api from '@/utils/api';
import { useCompletionsStore } from './completions';

const defaultApp = {
  created_at: "",
  updated_at: "",
  name: "",
  description: "",
  system_prompt: "You are a helpful, respectful and honest assistant. Always answer as helpfully as possible, while being safe.  Your answers should not include any harmful, unethical, racist, sexist, toxic, dangerous, or illegal content. Please ensure that your responses are socially unbiased and positive in nature.\n\nIf a question does not make any sense, or is not factually coherent, explain why instead of answering something not correct. If you don't know the answer to a question, please don't share false information.",
  model: {
    name: "llama2-chat",
    max_tokens: 64,
    temperature: 0.7,
    repeat_penalty: 1.0,
    repeat_penalty_last_n_tokens: 32,
    top_k: 40,
    top_p: 0.9,
  }
};

export const useAppsStore = defineStore('apps', () => {
  const loading = ref(false);
  const error = ref<string | null>(null);
  const apps = ref<App[]>([]);
  const app = ref<App>(defaultApp);

  const appCompletions = computed(() => {
    const completionsStore = useCompletionsStore();
    if (completionsStore.completions instanceof Map) {
      return completionsStore
        .completions
        .get(app.value.name) || [];
    }
    return [];
  });

  async function callApi(fn: () => Promise<void>) {
    loading.value = true;
    error.value = null;
    try {
      await fn();
    } catch (err: any) {
      error.value = err.message;
    } finally {
      loading.value = false;
    }
  }

  const fetchApps = () => callApi(async () => {
    apps.value = await api.fetchApps();
  });
  const fetchApp = (name: string) => callApi(async () => {
    app.value = await api.fetchApp(name);
  });
  const createApp = () => callApi(async () => {
    app.value = await api.createApp(app.value);
  });
  const updateApp = () => callApi(async () => {
    app.value = await api.updateApp(app.value);
  });
  const deleteApp = (name: string) => callApi(async () => {
    await api.deleteApp(name);
  });
  const resetApp = () => app.value = defaultApp;

  const createCompletion = (prompt: string) => callApi(async () => {
    const { createCompletion: inner } = useCompletionsStore();
    await inner(app.value, prompt);
  });

  return {
    loading,
    error,
    apps,
    app,
    appCompletions,
    fetchApps,
    fetchApp,
    createApp,
    updateApp,
    deleteApp,
    resetApp,
    createCompletion,
  };
});
