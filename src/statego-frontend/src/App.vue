<script setup lang="ts">
import { RouterView } from 'vue-router'
import NavBar from './components/NavBar.vue'
</script>

<template>
  <header class="flex fixed top-0 h-15 w-full bg-slate-700 ">
    <NavBar />
  </header>
  <div class="flex w-full bg-slate-300 h-screen justify-center">
    <RouterView  />
  </div>
  
</template>

<style scoped>
</style>
