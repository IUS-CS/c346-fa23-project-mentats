<script setup lang="ts">
import { RouterView } from 'vue-router';
import NavBar from './components/NavBar-Desktop.vue';
</script>

<template>
   <header class="fixed left-0 flex h-full w-20 bg-black">
      <NavBar />
   </header>
   <div class="flex h-screen items-center justify-center overflow-x-scroll bg-gray-300 pl-20">
      <RouterView />
   </div>
</template>

<style scoped></style>
