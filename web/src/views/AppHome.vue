<script lang="ts" setup>
import { onMounted } from "vue";
import { useRoute } from "vue-router";
import { useAppsStore } from "@/stores/apps";
import { storeToRefs } from "pinia";

const route = useRoute();
// const { app } = storeToRefs(useAppsStore());
const { fetchApp } = useAppsStore();

const promptResults = [
  {
    request: {

    },
    response: {

    }
  }
];

onMounted(() => {
  fetchApp(route.params.name as string);
});
</script>

<template>
  <div>Console</div>
</template>
