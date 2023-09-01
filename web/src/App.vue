<script setup lang="ts">
import { NavBar, NavLink } from "@/components/nav";
</script>

<template>
  <NavBar>
    <template #brand>
      <NavLink href="/">
        <img class="h-8 w-auto lg:block" src="@/assets/logo.svg" alt="LLM Explorer" />
      </NavLink>
    </template>
    <template #default>
      <div class="ml-10 flex items-baseline space-x-4">
        <NavLink href="/" active-class="bg-gray-900 text-white"
          inactive-class="text-gray-300 hover:bg-gray-700 hover:text-white"
          class="rounded-md px-3 py-2 text-sm font-medium">Home</NavLink>
        <NavLink href="/about" active-class="bg-gray-900 text-white"
          inactive-class="text-gray-300 hover:bg-gray-700 hover:text-white"
          class="rounded-md px-3 py-2 text-sm font-medium">About</NavLink>
      </div>
    </template>
    <template #nav-menu>
    </template>
  </NavBar>

<RouterView /></template>
