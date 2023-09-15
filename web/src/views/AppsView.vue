<script setup lang="ts">
import { NavLink } from "@/components/nav";
import { onMounted } from "vue";
import { useAppsStore } from "@/stores/apps";
import { storeToRefs } from "pinia";
import { useRouter } from "vue-router";

const router = useRouter();
const { apps } = storeToRefs(useAppsStore());
const { fetchApps } = useAppsStore();

const navAppHome = (name: string) => {
  router.push({ name: 'app-home', params: { name }});
};

onMounted(() => {
  fetchApps();
});
</script>

<template>
  <main>
    <div class="max-w-7xl py-6 sm:px-6 lg:px-8">
      <h1 class="text-3xl font-bold tracking-tight text-gray-900">This is the apps page!</h1>
      <div class="mt-6 flex gap-x-6">
        <NavLink href="/apps/new"
          class="rounded-md bg-seagreen-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
          New App</NavLink>
      </div>
      <ul role="list" class="divide-y divide-gray-100">
        <li v-for="app in apps" :key="app.name" class="flex justify-between gap-x-6 py-5 px-2.5 mt-3 rounded-md cursor-pointer hover:bg-gray-200" @click="navAppHome(app.name)">
          <div class="flex min-w-0 gap-x-4">
            <div class="min-w-0 flex-auto">
              <p class="text-sm font-semibold leading-6 text-gray-900">{{ app.name }}</p>
              <p class="mt-1 truncate text-xs leading-5 text-gray-500">{{ app.model.name }}</p>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </main>
</template>
