<script setup lang="ts">
import { RouterView } from 'vue-router';
import NavBar from './components/NavBar.vue';
</script>

<template>
   <div>
      <header class="h-15 fixed top-0 flex w-full bg-slate-700">
         <NavBar />
      </header>
      <div class="flex h-screen w-full bg-slate-300">
         <RouterView />
      </div>
   </div>
</template>

<style scoped></style>
