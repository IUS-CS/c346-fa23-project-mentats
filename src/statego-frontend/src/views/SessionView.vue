<script setup lang="ts">
import PlayerList from '@/components/PlayerList.vue'
import SessionNote from '@/components/SessionNote.vue'
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

// runs on mounted
// fetches session data from backend endpoint
onMounted(() => {})
</script>

<template>
  <div class="drop-shadow-3lg flex h-full w-5/6 flex-col bg-zinc-100 md:flex-row">
    <!-- left col -->
    <div class="flex h-full flex-col items-center justify-center gap-16 rounded-t-2xl md:w-2/5">
      <!-- images from session -->
      <div class="h-2/5 w-4/5">
        <img class="flex" :alt="imgAlt" src="https://placehold.co/600x400" />
      </div>
      <!-- time and name -->
      <div class="h-2/5 w-4/5">
        <h1 class="text-4xl">Game Name</h1>
        <h2 class="text-3xl">Campeign</h2>
        <p class="text-lg">Date/Time</p>
        <p class="text-lg">Length</p>
      </div>
    </div>

    <!-- right col -->
    <div class="flex h-full flex-col items-center justify-center gap-16 md:w-3/5">
      <!-- players / winner -->
      <div class="h-2/5 w-4/5">
        <PlayerList :players="playersList" />
      </div>
      <!-- session notes -->
      <div class="h-2/5 w-4/5">
        <SessionNote notes="session notes go here" />
      </div>
    </div>
  </div>
</template>
