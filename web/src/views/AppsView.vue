<script setup lang="ts">
import { NavLink } from "@/components/nav";
const apps = [
  {
    id: 1,
    name: 'my-first-prompt-app',
    description: 'This is my first prompt app!',
    prompts: [{
      type: "system",
      text: "Welcome to your new prompt app!",
    }, {
      type: "user",
      text: "Hello, world!",
    }],
    model: {
      name: "llama2-chat",
      options: {
        max_tokens: 75,
        temperature: 0.0,
        repeat_penalty: 1.1,
        repeat_penalty_last_n_tokens: 64,
        top_k: 0,
        top_p: 1.0,
      }
    }
  },
]
</script>

<template>
  <main>
    <div class="max-w-7xl py-6 sm:px-6 lg:px-8">
      <h1 class="text-3xl font-bold tracking-tight text-gray-900">This is the apps page!</h1>
      <div class="mt-6 flex gap-x-6">
        <NavLink href="/app/new"
          class="rounded-md bg-seagreen-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
          New App</NavLink>
      </div>
      <ul role="list" class="divide-y divide-gray-100">
        <li v-for="app in apps" :key="app.id" class="flex justify-between gap-x-6 py-5">
          <div class="flex min-w-0 gap-x-4">
            <!-- <img class="h-12 w-12 flex-none rounded-full bg-gray-50" :src="person.imageUrl" alt="" /> -->
            <div class="min-w-0 flex-auto">
              <p class="text-sm font-semibold leading-6 text-gray-900">{{ app.name }}</p>
              <p class="mt-1 truncate text-xs leading-5 text-gray-500">{{ app.model.name }}</p>
            </div>
          </div>
          <div class="hidden shrink-0 sm:flex sm:flex-col sm:items-end">
            <NavLink :href="{ name: 'app', params: { name: app.name } }"
              class="text-sm font-semibold leading-6 text-gray-900">
              View</NavLink>
          </div>
        </li>
      </ul>
    </div>
  </main>
</template>
