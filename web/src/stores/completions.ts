import { ref } from 'vue';
import { defineStore } from 'pinia';
import api from '@/utils/api';

interface AppCompletion extends Completion {
  app_name: string;
  created_at: string;
}

export const useCompletionsStore = defineStore('completions', () => {
  const loading = ref(false);
  const error = ref<string | null>(null);
  const completions = ref<AppCompletion[]>([]);

  const createCompletion = async (app: App, prompt: string): Promise<void> => {
    loading.value = true;
    try {
      const completion = await api.createCompletion(app, prompt);
      completions.value.push({
        ...completion,
        app_name: app.name,
        created_at: new Date().toISOString(),
      });
    } catch (e: any) {
      error.value = e.message;
    } finally {
      loading.value = false;
    }
  };

  return { loading, error, completions, createCompletion };
});
