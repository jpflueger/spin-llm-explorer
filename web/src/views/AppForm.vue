<script lang="ts" setup>
import { onMounted } from "vue";
import { useRoute } from "vue-router";
import { useAppsStore } from "@/stores/apps";
import { storeToRefs } from "pinia";

const route = useRoute();
const { app } = storeToRefs(useAppsStore());
const { fetchApp, initApp } = useAppsStore();

const modelOptions = [
  { name: "Llama2 Chat", value: "llama2-chat" },
  { name: "CodeLlama Instruct", value: "codellama-instruct" },
];

onMounted(() => {
  if (route.params.name) {
    fetchApp(route.params.name as string);
  } else {
    initApp();
  }
});
</script>

<template>
  <main>
    <div v-if="app" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
      <form>
        <div class="space-y-12">
          <div class="border-b border-gray-900/10 pb-12">
            <h2 class="text-base font-semibold leading-7 text-gray-900">Create a new Prompt App</h2>
            <p class="mt-1 text-sm leading-6 text-gray-600">This information will be displayed publicly so be careful what
              you share.</p>

            <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
              <div class="sm:col-span-2">
                <label for="name" class="block text-sm font-medium leading-6 text-gray-900">Name (must be url safe &
                  unique)</label>
                <div class="mt-2">
                  <input type="text" name="name" id="name" :value="app.name"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-4">
                <label for="description" class="block text-sm font-medium leading-6 text-gray-900">Desciption</label>
                <div class="mt-2">
                  <input type="text" name="description" id="description" :value="app.description"
                    placeholder="Simple chat assistant"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="col-span-full">
                <label for="system-prompt" class="block text-sm font-medium leading-6 text-gray-900">
                  System Prompt
                </label>
                <span class="text-sm leading-6 text-gray-600">Give the system some context on the task to be performed.
                </span>
                <div class="mt-2">
                  <textarea id="system-prompt" name="system-prompt" rows="3" v-model="app.system_prompt"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 cursor-text shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <!-- <div class="col-span-full">
                <label for="user-prompt" class="block text-sm font-medium leading-6 text-gray-900">
                  User Prompt
                </label>
                <span class="text-sm leading-6 text-gray-600">Provide a sample of what the user might input to your
                  application.
                </span>
                <div class="mt-2">
                  <textarea id="system-prompt" name="system-prompt" rows="3" v-model="userPrompt"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div> -->
            </div>
          </div>

          <div class="border-b border-gray-900/10 pb-12">
            <h2 class="text-base font-semibold leading-7 text-gray-900">Model Parameters</h2>
            <p class="mt-1 text-sm leading-6 text-gray-600">Select a model and provide default parameters.</p>

            <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">

              <div class="sm:col-span-full">
                <label for="model" class="block text-sm font-medium leading-6 text-gray-900">Model</label>
                <span class="text-sm leading-6 text-gray-600">Select one of the models supported by Spin.
                </span>
                <div class="mt-2">
                  <select id="model" name="model" v-model="app.model.name"
                    class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6">
                    <option v-for="option in modelOptions" :key="option.value" :value="option.value">
                      {{ option.name }}
                    </option>
                  </select>
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="max-tokens" class="block text-sm font-medium leading-6 text-gray-900">Max Tokens</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" name="max-tokens" id="max-tokens" :value="app.model.max_tokens"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="temp" class="block text-sm font-medium leading-6 text-gray-900">Temperature</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" name="temp" id="temp" :value="app.model.temperature"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="repeat-penalty" class="block text-sm font-medium leading-6 text-gray-900">Repeat
                  penalty</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" name="repeat-penalty" id="repeat-penalty" :value="app.model.repeat_penalty"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="repeat-penalty-last-n" class="block text-sm font-medium leading-6 text-gray-900">Repeat
                  penalty last N token count</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" name="repeat-penalty-last-n" id="repeat-penalty-last-n" :value="app.model.repeat_penalty_last_n_tokens"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="top-k" class="block text-sm font-medium leading-6 text-gray-900">Top K</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" name="top-k" id="top-k" :value="app.model.top_k"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="top-p" class="block text-sm font-medium leading-6 text-gray-900">Top P</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" name="top-p" id="top-p" :value="app.model.top_p"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

            </div>
          </div>

        </div>

        <div class="mt-6 flex items-center justify-end gap-x-6">
          <button type="button" class="text-sm font-semibold leading-6 text-gray-900">Cancel</button>
          <button type="submit"
            class="rounded-md bg-seagreen-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Save</button>
        </div>
      </form>
    </div>
  </main>
</template>
