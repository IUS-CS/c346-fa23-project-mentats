<script setup lang="ts">
import { RouterView } from 'vue-router';
import NavBar from './components/NavBar.vue';
</script>

<template>
   <header class="h-15 fixed top-0 flex w-full bg-slate-700">
      <NavBar />
   </header>
   <div class="flex h-screen items-center justify-center bg-gray-300 pt-16">
      <RouterView />
   </div>
</template>

<style scoped></style>
