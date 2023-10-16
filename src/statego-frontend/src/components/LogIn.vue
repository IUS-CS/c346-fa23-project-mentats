<script setup lang="ts">
import { ref } from 'vue';
import type { Ref } from 'vue';
let userName: Ref<string> = ref('');
let password: Ref<string> = ref('');
let showPass: Ref<string> = ref('password');
function togglePass() {
   if (showPass.value == 'password') {
      showPass.value = 'text';
   } else {
      showPass.value = 'password';
   }
}
</script>

<template>
   <div class="flex h-1/2 w-1/4 flex-col rounded-lg border-t-8 border-t-indigo-500 bg-gray-100 drop-shadow-2xl">
      <h1 class="flex w-full justify-center pb-12 font-Nunito-Sans text-3xl font-bold">Login</h1>
      <form class="flex w-full">
         <div class="flex h-full w-full flex-col items-center justify-center">
            <label class="w-4/5 font-Nunito-Sans text-xs" for="userName">Username or email</label>
            <input
               class="flex h-10 w-4/5 rounded-md border-2 border-solid border-gray-400 bg-gray-50 font-Nunito-Sans hover:bg-gray-100"
               v-model="userName"
            />

            <br />

            <label class="justify-left w-4/5 font-Nunito-Sans text-xs" for="password">Password</label>
            <input
               class="flex h-10 w-4/5 rounded-md border-2 border-solid border-gray-400 bg-gray-50 font-Nunito-Sans hover:bg-gray-100"
               :type="showPass"
               v-model="password"
            />
            <button @click.prevent="togglePass">
               <span class="icon">
                  <i class="fas" :class="{ 'fa-eye-slash': showPass, 'fa-eye': !showPass }"></i>
               </span>
            </button>
            <br />
            <div class="justify-left flex w-4/5">
               <button
                  class="flex h-10 w-2/5 items-center justify-center rounded-md bg-indigo-500 font-Nunito-Sans text-xl font-bold shadow-md hover:text-white hover:shadow-lg"
                  type="submit"
                  value="Log In"
               >
                  Log In
               </button>
            </div>

            <p class="justify-left w-4/5 pt-4 font-Nunito-Sans text-xs">
               Don't have an account yet?
               <a class="text-indigo-400 underline hover:text-indigo-600" href="/signup">Sign Up!</a>
            </p>
         </div>
      </form>
   </div>
</template>
