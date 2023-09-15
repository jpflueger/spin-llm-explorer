<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { useAppsStore } from "@/stores/apps";
import { storeToRefs } from "pinia";
import { TabGroup, TabList, TabPanels, TabPanel, Tab } from "@headlessui/vue";

const router = useRouter();
const { app, appCompletions, loading } = storeToRefs(useAppsStore());
const { fetchApp, createApp, updateApp, resetApp, createCompletion } = useAppsStore();

const modelOptions = [
  { name: "Llama2 Chat", value: "llama2-chat" },
  { name: "CodeLlama Instruct", value: "codellama-instruct" },
];

const prompt = ref("");

const submit = async (e: Event) => {
  e.preventDefault();
  if (router.currentRoute.value.name === "app-home") {
    await updateApp();
  } else {
    await createApp();
  }
};

const cancel = () => {
  resetApp();
  // router.back();
};

const sendConsolePrompt = async () => {
  await createCompletion(prompt.value);
  prompt.value = "";
};

onMounted(async () => {
  if (router.currentRoute.value.name === "app-home") {
    await fetchApp(router.currentRoute.value.params.name as string);
  }
});
</script>

<template>
  <main>
    <div v-if="app" class="mx-auto max-w-7xl py-6 sm:px-6 lg:px-8">
      <TabGroup>
        <TabList>
          <Tab as="template" v-slot="{ selected }">
            <button class="px-6 py-3 rounded-md" :class="{ 'bg-gray-500 text-white': selected }">App Info</button>
          </Tab>
          <Tab as="template" v-slot="{ selected }">
            <button class="px-6 py-3 rounded-md" :class="{ 'bg-gray-500 text-white': selected }">Model Params</button>
          </Tab>
          <Tab as="template" v-slot="{ selected }">
            <button class="px-3 py-3 rounded-md" :class="{ 'bg-gray-500 text-white': selected }">Console</button>
          </Tab>
        </TabList>
        <TabPanels>
          <TabPanel>
            <form @submit="submit" class="mt-10 h-full grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
              <div class="sm:col-span-2">
                <label for="name" class="block text-sm font-medium leading-6 text-gray-900">Name (must be url safe &
                  unique)</label>
                <div class="mt-2">
                  <input type="text" name="name" id="name" v-model="app.name"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-4">
                <label for="description" class="block text-sm font-medium leading-6 text-gray-900">Desciption</label>
                <div class="mt-2">
                  <input type="text" name="description" id="description" v-model="app.description"
                    placeholder="Simple chat assistant"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="col-span-full">
                <label for="system-prompt" class="block text-sm font-medium leading-6 text-gray-900">
                  System Prompt
                </label>
                <span class="text-sm leading-6 text-gray-600">Give the system some context on the task to be
                  performed.
                </span>
                <div class="mt-2">
                  <textarea id="system-prompt" name="system-prompt" rows="3" v-model="app.system_prompt"
                    class="block min-h-fit w-full rounded-md border-0 px-3 py-1.5 text-gray-900 cursor-text shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
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

              <div class="sm:col-span-full flex items-center justify-end gap-x-6">
                <button type="button" @click="cancel"
                  class="text-sm font-semibold leading-6 text-gray-900">Cancel</button>
                <button type="submit"
                  class="rounded-md bg-seagreen-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Save</button>
              </div>
            </form>
          </TabPanel>
          <TabPanel>
            <form @submit="submit" class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
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
                  <input type="number" step="any" name="max-tokens" id="max-tokens" v-model="app.model.max_tokens"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="temp" class="block text-sm font-medium leading-6 text-gray-900">Temperature</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" step="any" name="temp" id="temp" v-model="app.model.temperature"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="repeat-penalty" class="block text-sm font-medium leading-6 text-gray-900">Repeat
                  penalty</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" step="any" name="repeat-penalty" id="repeat-penalty"
                    v-model="app.model.repeat_penalty"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="repeat-penalty-last-n" class="block text-sm font-medium leading-6 text-gray-900">Repeat
                  penalty last N token count</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" step="any" name="repeat-penalty-last-n" id="repeat-penalty-last-n"
                    v-model="app.model.repeat_penalty_last_n_tokens"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="top-k" class="block text-sm font-medium leading-6 text-gray-900">Top K</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" step="any" name="top-k" id="top-k" v-model="app.model.top_k"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-3">
                <label for="top-p" class="block text-sm font-medium leading-6 text-gray-900">Top P</label>
                <span class="text-sm leading-6 text-gray-600">TODO
                </span>
                <div class="mt-2">
                  <input type="number" step="any" name="top-p" id="top-p" v-model="app.model.top_p"
                    class="block w-full rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                </div>
              </div>

              <div class="sm:col-span-full flex items-center justify-end gap-x-6">
                <button type="button" @click="cancel"
                  class="text-sm font-semibold leading-6 text-gray-900">Cancel</button>
                <button type="submit"
                  class="rounded-md bg-seagreen-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">Save</button>
              </div>
            </form>
          </TabPanel>
          <TabPanel as="div" class="mt-10">
            <div v-if="!app.created_at">
              <h3>The app must be saved before the Console is enabled</h3>
            </div>
            <div v-if="app.created_at" class="flex flex-col">
              <div class="flex flex-row gap-x-3">
                <input type="text" name="console-input" id="console-input" v-model="prompt" @keyup.enter="sendConsolePrompt"
                  class="grow rounded-md border-0 px-3 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" />
                <input type="button" value="Send" @click="sendConsolePrompt"
                  class="basis-32 rounded-md bg-seagreen-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600" />
              </div>
              <div role="status" class="flex flex-row content-center items-center p-3 h-12" :class="[loading ? 'visible' : 'invisible']">
                  <svg aria-hidden="true" :class="[loading ? 'animate-spin' : '']" class="w-4 h-4 mr-2 text-gray-200 dark:text-gray-600 fill-oxfordblue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                    <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/></svg>
                  <span>Generating...</span>
              </div>
              <div class="flex flex-col gap-y-6">
                <div v-for="ac in appCompletions" :key="ac.created_at" class="flex flex-col bg-white border-2 rounded-md border-rust-500">
                  <div class="bg-rust-500 relative">
                    <p class="text-center scroll-py-1.5 text-white font-bold rounded-md">System Prompt</p>
                    <p class="text-white whitespace-pre-line my-3 mx-6 select-all">{{ ac.app_snapshot.system_prompt }}</p>
                  </div>
                  <div class="py-1.5 px-1.5 flex flex-nowrap gap-3 select-none">
                    <span class="p-1 text-xs text-white rounded-md bg-rust-200">Model: {{ ac.app_snapshot.model.name }}</span>
                    <span class="p-1 text-xs text-white rounded-md bg-rust-200">Temperature: {{ ac.app_snapshot.model.temperature }}</span>
                    <span class="p-1 text-xs text-white rounded-md bg-rust-200">Max Tokens: {{ ac.app_snapshot.model.max_tokens }}</span>
                    <span class="p-1 text-xs text-white rounded-md bg-rust-200">Repeat Penalty: {{ ac.app_snapshot.model.repeat_penalty }}</span>
                    <span class="p-1 text-xs text-white rounded-md bg-rust-200">Repeat Penalty Last N: {{ ac.app_snapshot.model.repeat_penalty_last_n_tokens }}</span>
                    <span class="p-1 text-xs text-white rounded-md bg-rust-200">Top K: {{ ac.app_snapshot.model.top_k }}</span>
                    <span class="p-1 text-xs text-white rounded-md bg-rust-200">Top P: {{ ac.app_snapshot.model.top_p }}</span>
                    <span class="p-1 text-xs text-white rounded-md bg-rust-200">Prompt Tokens: {{ ac.completion.usage?.prompt_token_count }}</span>
                    <span class="p-1 text-xs text-white rounded-md bg-rust-200">Generated Tokens: {{ ac.completion.usage?.generated_token_count }}</span>
                  </div>
                  <div class="self-start my-3 mx-6 max-w-lg">
                    <p class="py-3 px-6 rounded-md bg-oxfordblue-400 whitespace-pre-line text-white select-all">{{ ac.user_prompt }}</p>
                    <p class="mt-1 text-left text-sm text-gray-600">User</p>
                  </div>
                  <div class="self-end my-3 mx-6 max-w-lg">
                    <p class="py-3 px-6 rounded-md bg-gray-100 whitespace-pre-line select-all">{{ ac.completion.messages[0].content }}</p>
                    <p class="text-right text-sm text-gray-600">Assistant</p>
                  </div>
                </div>
              </div>
            </div>
          </TabPanel>
        </TabPanels>
      </TabGroup>
    </div>
  </main>
</template>
