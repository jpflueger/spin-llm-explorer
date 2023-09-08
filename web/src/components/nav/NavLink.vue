<script setup lang="ts">
import type { RouteLocationRaw } from 'vue-router';
const props = withDefaults(defineProps<{
  href: RouteLocationRaw;
  activeClass?: string;
  inactiveClass?: string;
  ariaCurrentValue?: 'page' | 'step' | 'location' | 'date' | 'time' | 'true' | 'false';
}>(), {
  ariaCurrentValue: 'page',
});
</script>

<template>
  <RouterLink
    v-slot="{ href, isActive, isExactActive, navigate }"
    :to="href"
    custom
  >
    <a
      v-bind="$attrs"
      :href="href"
      :class="isActive ? activeClass : inactiveClass"
      :aria-current="isExactActive ? ariaCurrentValue : undefined"
      @click="navigate"
      >
      <slot />
    </a>
  </RouterLink>
</template>
