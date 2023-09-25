<script setup lang="ts">
const props = defineProps({
  winner: { type: String, required: false },
  players: { type: Array<String>, required: true }
})
</script>

<template>
  <div class="flex flex-col h-full rounded-2xl outline outline-1 outline-gray-300">
    <!-- Winner of the Session if there is one -->
    <div class="flex h-1/5 w-full bg-purple-400 rounded-t-2xl">
      <img src="https://placehold.co/100x100" class="rounded-tl-2xl" />
      <h2 class="text-2xl p-3">Winner</h2>
    </div>
    <!-- list of all players not including the winner -->
    <div class="flex flex-col h-4/5 w-full overflow-y-scroll gap-3 pt-1 pb-3 pl-3 pr-3">
      <div
        v-for="(item, index) in props.players"
        :key="index"
        class="flex h-1/5 even:bg-gray-100 rounded-md outline outline-1 outline-gray-300"
      >
        <img src="https://placehold.co/100x100" class="" />
        <p class="p-3">{{ item }}</p>
      </div>
    </div>
  </div>
</template>
