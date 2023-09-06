<script setup lang="ts">
let x: number = 10;
</script>

<template>
   <div class="flex h-full w-full items-center justify-center">
      <p class="text-red-700">Hello This is the Statego Landing page</p>
      <p>the value of x: {{ x }} is bound to the variable value</p>
   </div>
</template>
