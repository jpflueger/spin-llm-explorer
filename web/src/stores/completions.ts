import { ref } from 'vue';
import { defineStore, type StateTree } from 'pinia';
import api from '@/utils/api';

export const useCompletionsStore = defineStore('completions', () => {
  const loading = ref(false);
  const error = ref<string | null>(null);
  const completions = ref(new Map<string, AppCompletion[]>());

  const clearCompletions = (appName: string): void => {
    completions.value.set(appName, []);
  };

  const createCompletion = async (app: App, prompt: string): Promise<void> => {
    loading.value = true;
    try {
      //TODO: probably a more performant way to do this but this is fine for now
      const app_snapshot = JSON.parse(JSON.stringify(app));
      const completion = await api.createCompletion(app, prompt);
      const app_completion = {
        created_at: new Date().toISOString(),
        user_prompt: prompt,
        app_snapshot,
        completion,
      };

      //TODO: there's probably a library that helps do atomic map updates
      const app_completions = completions.value instanceof Map
        ? completions.value.get(app.name) || [] : [];
      app_completions.push(app_completion);
      completions.value.set(app.name, app_completions);
    } catch (e: any) {
      error.value = e.message;
    } finally {
      loading.value = false;
    }
  };

  return { loading, error, completions, createCompletion, clearCompletions };
}, {
  persist: {
    debug: true,
    storage: localStorage,
    serializer: {
      serialize: (value: StateTree) => {
        return JSON.stringify(value, (_, value) => {
          if (value instanceof Map) {
            return {
              dataType: 'Map',
              value: Array.from(value.entries()),
            };
          } else {
            return value;
          }
        });
      },
      deserialize: (value: string) => {
        return JSON.parse(value, (_, value) => {
          if (value && value.dataType === 'Map') {
            return new Map(value.value);
          } else {
            return value;
          }
        });
      }
    }
  }
});
