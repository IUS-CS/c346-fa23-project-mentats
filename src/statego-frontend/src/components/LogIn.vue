<script setup lang="ts">
import { ref } from 'vue'
import type { Ref } from 'vue'
let userName: Ref<string> = ref('')
let password: Ref<string> = ref('')
let showPass: Ref<string> = ref('password')
function togglePass() {
  if (showPass.value == 'password') {
    showPass.value = 'text'
  } else {
    showPass.value = 'password'
  }
}
</script>

<template>
  <div
    class="flex flex-col w-1/4 h-1/2 bg-gray-100 rounded-lg drop-shadow-2xl border-t-indigo-500 border-t-8"
  >
    <h1 class="flex text-3xl font-bold w-full justify-center pb-12">Login</h1>
    <form class="flex w-full">
      <div class="flex flex-col w-full h-full items-center justify-center">
        <label class="w-4/5 text-xs" for="userName">Username or email</label>
        <input
          class="hover:bg-gray-100 flex w-4/5 h-10 rounded-md bg-gray-50 border-2 border-solid border-gray-400"
          v-model="userName"
        />

        <br />

        <label class="justify-left w-4/5 text-xs" for="password">Password</label>
        <input
          class="hover:bg-gray-100 flex w-4/5 h-10 rounded-md bg-gray-50 border-2 border-solid border-gray-400"
          :type="showPass"
          v-model="password"
        />
        <button @click.prevent="togglePass"><span class="icon">
            <i class="fas" :class="{ 'fa-eye-slash': showPass, 'fa-eye': !showPass }"></i>
          </span></button>
        <br />
        <div class="flex w-4/5 justify-left">
          <button
            class="justify-center items-center flex w-2/5 h-10 text-xl bg-indigo-500 rounded-md shadow-md font-bold hover:shadow-lg hover:text-white"
            type="submit"
            value="Log In"
          >
            Log In
          </button>
        </div>

        <p class="text-xs pt-4 w-4/5 justify-left">
          Don't have an account yet?
          <a class="text-indigo-400 underline hover:text-indigo-600" href="">Sign Up!</a>
        </p>
      </div>
    </form>
  </div>
</template>

<style scoped>
/* div {
        outline: 2px;
        outline-color: rgb(0, 0, 0);
        outline-style: solid;
    }
    .hide{
  display:none;
}
    .show{
  display:block;
} */
</style>
