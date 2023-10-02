<script setup lang="ts">
const props = defineProps({
  notes: { type: String, required: true }
})
</script>

<template>
  <div class="">
    <p>{{ notes }}</p>
  </div>
</template>
