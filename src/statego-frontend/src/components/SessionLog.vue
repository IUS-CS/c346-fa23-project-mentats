<script setup lang="ts">
import { ref } from 'vue';
import type { Ref } from 'vue';
let gameName: Ref<string> = ref('');
let playerName1: Ref<String> = ref('');
let playerName2: Ref<String> = ref('');
let winnerName: Ref<String> = ref('');
let description: Ref<String> = ref('');
function addPlayer() {}
</script>

<template>
   <div class="drop-shadow-2xlshadow-2xl flex h-5/6 w-3/5 flex-col justify-center rounded-lg border-t-8 border-t-indigo-500 bg-gray-100 pb-12 pt-12 overflow-auto">
      <h1 class="flex w-full justify-center pb-5 font-Nunito-Sans text-2xl font-bold">Session Log</h1>
      <div class="flex h-full w-full flex-row">
         <div class="flex h-full w-2/3 flex-col pl-20 pr-20">
            <p class="pb-3 pl-2 pt-4 font-Nunito-Sans text-lg font-semibold">Game name</p>
            <p class="pb-8 pl-2 font-Nunito-Sans text-lg font-semibold">Start</p>
            <p class="pb-6 pl-2 pt-1 font-Nunito-Sans text-lg font-semibold">End</p>
            <p class="pb-6 pl-2 pt-1 font-Nunito-Sans text-lg font-semibold">Player 1</p>
            <p class="pb-5 pl-2 font-Nunito-Sans text-lg font-semibold">Player 2</p>
            <p class="pb-5 pl-2 font-Nunito-Sans text-lg font-semibold">Complete?</p>
            <p class="pb-5 pl-2 font-Nunito-Sans text-lg font-semibold">Winner</p>
            <p class="pb-6 pl-2 font-Nunito-Sans text-lg font-semibold">Last Session of Campaign</p>
            <p class="pb-20 pl-2 font-Nunito-Sans text-lg font-semibold">Description</p>
            <p class="pl-2 pt-7 font-Nunito-Sans text-lg font-semibold">Image</p>
         </div>

         <div class="flex h-full w-2/3 flex-col pl-20 pr-2 pt-1">
            <form class="flex h-full w-fit flex-col">
               <!-- game name -->
               <div class="pb-3 pr-3">
                  <input class="font-Nunito-Sans italic rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 text-center hover:bg-gray-100 pr-3" v-model="gameName" />
               </div>
               <!-- date and time started -->
               <div class="flex flex-row pb-3">
                  <div class="pb-3 pr-3">
                     <input
                        class="rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 pl-6 pr-10 font-Nunito-Sans font-semibold hover:shadow-md"
                        type="date"
                        id="dateStarted"
                        name="dateStarted"
                     />
                  </div>

                  <legend class="text-md font-semibold">
                     <input
                        class="rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 pl-10 pr-14 font-Nunito-Sans hover:shadow-md"
                        type="time"
                        name="startTime"
                     />
                  </legend>
               </div>

               <!-- date and time ended -->
               <div class="flex flex-row pb-3">
                  <div class="pb-3 pr-3">
                     <input
                        class="rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 pl-6 pr-10 font-Nunito-Sans font-semibold hover:shadow-md"
                        type="date"
                        id="dateEnded"
                        name="dateEnded"
                     />
                  </div>
                  <legend class="text-md font-semibold">
                     <input
                        class="rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 pl-10 pr-14 font-Nunito-Sans hover:shadow-md"
                        type="time"
                        name="endTime"
                     />
                  </legend>
               </div>

               <!-- player name text boxes -->
               <div class="flex pb-3">
                  <input
                     class="font-Nunito-Sans rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 text-center hover:bg-gray-100 pr-3"
                     v-model="playerName1"
                  />
               </div>
               <div class="flex pb-3">
                  <input
                     class="font-Nunito-Sans rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 text-center hover:bg-gray-100 pr-3"
                     v-model="playerName2"
                  />
               </div>

               <!-- yes or no dropdown -->
               <div class="pb-3">
                  <select class="font-Nunito-Sans rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 hover:shadow-md" onchange="yesorno(this.value)">
                     <option value="Choose" selected="true">Y/N</option>
                     <option value="yes">Yes</option>
                     <option value="no">No</option>
                  </select>
               </div>

               <!-- winner textbox -->
               <div class="pb-6">
                  <input class="font-Nunito-Sans rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 text-center hover:bg-gray-100 pr-3" v-model="winnerName" />
               </div>

               <!-- date and time of last session  -->
               <div class="flex flex-row pb-8">
                  <div class="pb-3 pr-3">
                     <input
                        class="rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 pl-6 pr-10 font-Nunito-Sans font-semibold hover:shadow-md"
                        type="date"
                        id="lastSession"
                        name="lastSession"
                     />
                  </div>
                  <legend>
                     <input
                        class="rounded-md border-2 border-solid border-gray-400 bg-gray-50 p-1 pl-10 pr-14 font-Nunito-Sans font-semibold hover:shadow-md"
                        type="time"
                        name="lastTime"
                     />
                  </legend>
               </div>

               <!-- description textbox -->
               <div class="pb-3">
                  <textarea
                     class="font-Nunito-Sans rounded-md border-2 border-solid border-gray-400 bg-gray-50 pb-16 pr-3 hover:shadow-md"
                     id="descriptionText"
                     name="descriptionText"
                  ></textarea>
               </div>

               <!-- image upload button -->
               <input
                  class="font-Nunito-Sans pb-10 file:rounded-md file:border-2 file:border-solid file:border-gray-400 file:bg-gray-50 file:p-1 file:hover:bg-gray-100 file:hover:shadow-md"
                  type="file"
                  name="pictureUpload"
                  accept="image/*"
               />

               <!-- submit button -->
               <div class="pt-10">
                  <button
                     class="flex h-10 w-1/4 items-center justify-center rounded-md bg-indigo-500 pl-16 pr-16 font-Nunito-Sans text-xl font-bold shadow-md hover:text-white hover:shadow-lg"
                     type="submit"
                     value="Submit"
                  >
                     Submit
                  </button>
               </div>
            </form>
         </div>
      </div>
   </div>
</template>
