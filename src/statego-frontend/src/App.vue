<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <header class="flex fixed top-0 h-16 w-full bg-gray-500">
    <NavBar />
  </header>
  <div class="flex h-screen bg-gray-300 justify-center pt-16">
    <RouterView />
  </div>
</template>

<style scoped></style>
