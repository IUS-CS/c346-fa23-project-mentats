<script setup lang="ts">
import { ref } from 'vue'
import type { Ref } from 'vue'
let gameName: Ref<string> = ref('')
let playerName1: Ref<String> = ref('')
let playerName2: Ref<String> = ref('')
let winnerName: Ref<String> = ref('')
let description: Ref<String> = ref('')
function addPlayer() {}
</script>

<template>
  <div
    class="flex flex-col w-2/5 h-4/5 bg-gray-100 rounded-lg drop-shadow-2xlshadow-2xl border-t-indigo-500 border-t-8 pt-12 pb-12"
  >
    <form class="flex flex-co w-full h-full pb-2 pt-2">
      <div class="flex flex-col w-full h-full items-center justify-center">
        <!-- headings -->
        <h1 class="flex text-2xl font-bold w-full justify-center pb-4">Session Log</h1>
        <form>
          <label class="text-md font-semibold" for="gameName">Game name </label>
          <input
            class="hover:bg-gray-100 rounded-md bg-gray-50 border-2 border-gray-400 border-solid p-1 italic text-lg text-center"
            v-model="gameName"
          />
        </form>

        <!-- date and time started label and calendar -->
        <form class="flex pb-2 pt-2">
          <label class="text-md font-semibold p-2" for="dateStarted">Start</label>
          <input
            class="hover:shadow-md rounded-md bg-gray-50 border-2 border-gray-400 border-solid"
            type="date"
            id="dateStarted"
            name="dateStarted"
          />
          <legend class="text-md font-semibold">
            <input
              class="hover:shadow-md rounded-md bg-gray-50 border-2 border-gray-400 border-solid p-1.5"
              type="time"
              name="startTime"
            />
          </legend>
        </form>

        <!-- date and time ended label and calendar -->
        <form class="flex pb-2 pt-2">
          <label class="text-md font-semibold p-2" for="dateEnded">End</label>
          <input
            class="hover:shadow-md rounded-md bg-gray-50 border-2 border-gray-400 border-solid"
            type="date"
            id="dateEnded"
            name="dateEnded"
          />
          <legend class="text-md font-semibold">
            <input
              class="hover:shadow-md rounded-md bg-gray-50 border-2 border-gray-400 border-solid p-1.5"
              type="time"
              name="endTime"
            />
          </legend>
        </form>

        <!-- player names labels and text boxes -->
        <form class="pb-2 pt-2">
          <label class="text-md font-semibold pb-2 pt-2 pr-2" for="playerName1">Player 1</label>
          <input
            class="hover:bg-gray-100 rounded-md bg-gray-50 border-2 border-gray-400 border-solid p-1 text-center"
            v-model="playerName1"
          />
          <label class="text-md font-semibold pb-2 pt-2 pr-2 pl-2" for="playerName2"
            >Player 2</label
          >
          <input
            class="hover:bg-gray-100 rounded-md bg-gray-50 border-2 border-gray-400 border-solid p-1 text-center"
            v-model="playerName2"
          /><br />
        </form>

        <!-- dropdown menu asking if game was completed with winner's name textbox -->
        <form class="pb-2 pt-2">
          <label class="text-md font-semibold pl-2 pr-2" for="yesorno">Complete?</label>
          <select
            class="hover:shadow-md rounded-md bg-gray-50 border-2 border-solid border-gray-400 p-1"
            onchange="yesorno(this.value)"
          >
            <option value="Choose" selected="true">Y/N</option>
            <option value="yes">Yes</option>
            <option value="no">No</option>
          </select>
          <label class="text-md font-semibold p-2" for="winner">Winner</label>
          <input
            class="hover:bg-gray-100 rounded-md bg-gray-50 border-2 border-gray-400 border-solid p-1 text-center"
            v-model="winnerName"
          />
        </form>

        <!-- last session played calendar and timePicker -->
        <form class="flex pb-2 pt-2">
          <label class="text-md font-semibold p-2" for="lastSession"
            >Last session of campaign
          </label>
          <input
            class="hover:shadow-md rounded-md bg-gray-50 border-2 border-gray-400 border-solid"
            type="date"
            id="lastSession"
            name="lastSession"
          />
          <legend>
            <input
              class="hover:shadow-md rounded-md bg-gray-50 border-2 border-gray-400 border-solid p-1.5"
              type="time"
              name="startTime"
            />
          </legend>
        </form>

        <!-- description text box -->
        <form class="pb-2 pt-2">
          <label class="align-top text-md font-semibold p-2" for="description">Description </label>
          <textarea
            class="hover:shadow-md rounded-md bg-gray-50 border-2 border-gray-400 border-solid pb-22 pr-32"
            id="descriptionText"
            name="descriptionText"
          ></textarea>
        </form>

        <!-- image upload -->
        <div class="flex flex-col justify-center items-center">
          <form class="p-2">
            <input
              class=" file:hover:shadow-md file:hover:bg-gray-100 pb-4 file:rounded-md file:p-1 file:bg-gray-50 file:border-gray-400 file:border-2 file:border-solid"
              type="file"
              name="pictureUpload"
              accept="image/*"
            />
          </form>
        </div>

        <!-- submit button -->
        <button
          class="justify-center items-center flex w-1/4 h-14 text-xl bg-indigo-500 rounded-md shadow-md font-bold hover:shadow-lg hover:text-white"
          type="submit"
          value="Submit"
        >
          Submit
        </button>
      </div>
    </form>
  </div>
</template>

<style scoped>
/* div {
  outline: 4px;
  outline-color: rgb(0, 0, 0);
  outline-style: solid;
}
.hide {
  display: none;
}
.show {
  display: block;
}*/
</style>
