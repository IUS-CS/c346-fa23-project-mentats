<script setup lang="ts">
import PlayerList from '@/components/PlayerList.vue'
import { ref, onMounted } from 'vue'
import type { Ref } from 'vue'
let imgAlt: Ref<string> = ref('Game Board')

const playersList: Array<String> = [
  'John Brown',
  'Tyler Popson',
  'Mason Napper',
  'Zachary Nelson',
  'John Brown',
  'Tyler Popson',
  'Mason Napper',
  'Zachary Nelson'
]

onMounted(() => {})
</script>

<template>
  <div class="flex flex-col md:flex-row w-5/6 h-full bg-gray-200">
    <!-- left col -->
    <div class="flex flex-col md:w-2/5 h-full items-center justify-center gap-16 rounded-t-2xl">
      <!-- images from session -->
      <div class="w-4/5 h-2/5">
        <img class="flex" :alt="imgAlt" src="https://placehold.co/600x400" />
      </div>
      <!-- time and name -->
      <div class="w-4/5 h-2/5">
        <h1 class="text-4xl">Game Name</h1>
        <h2 class="text-3xl">Campeign</h2>
        <p class="text-lg">Date/Time</p>
        <p class="text-lg">Length</p>
      </div>
    </div>

    <!-- right col -->
    <div class="flex flex-col md:w-3/5 h-full items-center justify-center gap-16">
      <!-- players / winner -->
      <div class="w-4/5 h-2/5">
        <PlayerList :players="playersList" />
      </div>
      <!-- session notes -->
      <div class="w-4/5 h-2/5"></div>
    </div>
  </div>
</template>

<style scoped>
/* div{
    outline: 2px;
    outline-color: black;
    outline-style: solid;
} */
</style>
