<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { useRoute } from "vue-router";
import { useAppsStore } from "@/stores/apps";
import { storeToRefs } from "pinia";

const route = useRoute();

// apps store
const store = useAppsStore();
const { loading: loading, app, appCompletions } = storeToRefs(store);
const { fetchApp, createCompletion } = store;

const prompt = ref<string>("");

onMounted(() => {
  fetchApp(route.params.name as string);
});
</script>

<template>
  <div>{{ app.name }} Console</div>
  <div>
    <input type="text" v-model="prompt" />
    <button @click="createCompletion(prompt)">Send</button>
  </div>
  <div v-if="loading">Loading...</div>
  <div v-for="c in appCompletions" :key="c.created_at">
    {{ c.messages[0].content }}
  </div>
</template>
