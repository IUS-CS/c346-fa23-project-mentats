<script setup lang="ts">
const props = defineProps({
   winner: { type: String, required: false },
   players: { type: Array<String>, required: true },
});
</script>

<template>
   <div class="flex h-full flex-col rounded-2xl bg-zinc-100 outline outline-1 outline-zinc-300 drop-shadow-md">
      <!-- Winner of the Session if there is one -->
      <div class="flex h-1/4 w-full rounded-t-2xl bg-purple-400">
         <img src="https://placehold.co/100x100" class="rounded-tl-2xl" />
         <h2 class="p-3 text-2xl">Winner</h2>
      </div>
      <!-- list of all players not including the winner -->
      <div class="flex h-3/4 w-full flex-col gap-3 overflow-y-scroll pb-3 pl-3 pr-3 pt-1">
         <div v-for="(item, index) in props.players" :key="index" class="flex h-1/3 rounded-md outline outline-1 outline-zinc-300 even:bg-zinc-200">
            <img src="https://placehold.co/100x100" class="" />
            <p class="p-3">{{ item }}</p>
         </div>
      </div>
   </div>
</template>
