<script setup lang="ts">
import { RouterLink } from 'vue-router';
</script>

<template>
   <nav class="flex h-full w-full items-center justify-center">
      <RouterLink class="p-5 text-white hover:bg-slate-900" to="/">Home</RouterLink>
      <RouterLink class="p-5 text-white hover:bg-slate-900" to="/about">About</RouterLink>
      <RouterLink class="p-5 text-white hover:bg-slate-900" to="/logsession">Log Session</RouterLink>
      <RouterLink class="p-5 text-white hover:bg-slate-900" to="/login">Login</RouterLink>
   </nav>
</template>
