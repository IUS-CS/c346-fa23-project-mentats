<script setup lang="ts">
    import { ref } from 'vue';
    import type {Ref} from 'vue';
    let gameName: Ref<string> = ref("");
    let playerName1: Ref<String> = ref("")
    let playerName2: Ref<String> = ref("")
    let playerName3: Ref<String> = ref("")
    let playerName4: Ref<String> = ref("")
    let winnerName: Ref<String> = ref("")
    let description: Ref<String> = ref("")
</script>

<template>
  <form>
    <div class="flex flex-col w-full h-full items-center justify-center">
        <p>Greetings player.</p>
        <p>Please log your session below:</p>
        <form>
          <label for="gameName">Game name: </label>
          <input v-model="gameName"/>
        </form>

        <form>
          <label for="dateStarted">Date started</label>
          <input type="date" id="dateStarted" name="dateStarted">
          <!-- <button type="submit">Submit</button> -->
        </form>
        <form action="#">
          <legend>
            Start time:
            <input type="time" name="startTime">
          </legend>
        <!-- <input type="submit" value="Submit"> -->
        </form>
        <form>
          <label for="dateEnded">Date ended</label>
          <input type="date" id="dateEnded" name="dateEnded">
        </form>
        <form action="#">
          <legend>
            End time:
            <input type="time" name="endTime">
          </legend>
        </form>
        
        <form>
          <label for="playerName1">Player 1: </label>
          <input v-model="playerName1"/><br>
          <label for="playerName2">Player 2:</label>
          <input v-model="playerName2"/><br>
          <label for="playerName3">Player 3:</label>
          <input v-model="playername3"/><br>
          <label for="playerName4">Player 4:</label>
          <input v-model="playername4"/><br>
        </form>
        
        <form>
          <label for="yesorno">Complete?</label>
          <select onchange="yesorno(this.value)">
          <option value="Choose" selected="selected">Y/N</option>
          <option value="yes" >Yes</option>
          <option value="no" >No</option>
        </select>
        </form>

        <form>
          <label for="winner">If yes, who was the winner: </label>
          <input v-model="winnerName"/>
        </form>

        <form action="#">
          <label for="lastSession">Last session of campaign: </label><br>
          <input type="date" id="lastSession" name="lastSession">
          <legend>
            <input type="time" name="startTime">
          </legend>
        </form>
        <form>
          <label for="description">Description: 
            <input v-model="description"/>
          </label>
        </form>

        <form>
          <label for="pictureUpload">Image: </label>
          <input type = "file" name = "pictureUpload" accept = "image/*" />
        </form>
      <input type="submit" value="Submit">
    </div>
  </form>
</template>


<style scoped> 
    div {
        outline: 4px;
        outline-color: rgb(0, 0, 0);
        outline-style: solid;
    }
    .hide{
  display:none;
}
.show{
  display:block;
}
</style>