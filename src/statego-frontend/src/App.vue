<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <header class="fixed top-0 flex h-16 w-full bg-gray-500">
    <NavBar />
  </header>
  <div class="flex h-screen items-center justify-center bg-gray-300 pt-16">
    <RouterView />
  </div>
</template>

<style scoped></style>
