<script setup lang="ts">
import { RouterLink } from 'vue-router';
</script>

<template>
   <nav class="flex h-full w-full flex-col items-center">
      <RouterLink class="h-20 w-20 p-5 text-white transition-colors hover:text-purple-400" to="/">
         <v-icon name="la-home-solid" scale="2" />
      </RouterLink>
      <RouterLink class="p-5 text-white transition-colors hover:text-purple-400" to="/about">
         <v-icon name="la-info-circle-solid" scale="2" />
      </RouterLink>
      <RouterLink class="p-5 text-white transition-colors hover:text-purple-400" to="/logsession">
         <v-icon name="la-plus-solid" scale="2" />
      </RouterLink>
      <div class="flex h-full flex-initial flex-col justify-end">
         <RouterLink class="h-20 w-20 p-5 text-white transition-colors hover:text-purple-400" to="/login">
            <v-icon name="la-arrow-right-solid" scale="2" />
         </RouterLink>
      </div>
   </nav>
</template>
