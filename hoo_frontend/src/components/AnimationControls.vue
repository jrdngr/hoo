<template>
  <div id="animation">
    <div class="control">
      <button @click="rotate">Rotate</button>
      <button @click="random">Random</button>
      <button @click="stop">Stop</button>
    </div>
    <div class="control">
      <input id="trans-time" type="number" min="0" max="65535" v-model="transitionTime">
      <label for="trans-time">Transition time</label>
    </div>
    <div class="control">
      <input id="hold-time" type="number" min="0" max="65535" v-model="holdTime">
      <label for="hold-time">Hold time</label>
    </div>
  </div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import { BASE_URL } from "../Hoo.vue";

@Component
export default class AnimationControls extends Vue {
    private transitionTime: number = 10;
    private holdTime: number = 0;

    private rotate(event: any) {
      const url = `${BASE_URL}/rotate/${this.transitionTime}/${this.holdTime}`;
      fetch(url);
    }

    private random(event: any) {
      const url = `${BASE_URL}/random/${this.transitionTime}/${this.holdTime}`;
      fetch(url);
    }

    private stop(event: any) {
      const url = `${BASE_URL}/stop`;
      fetch(url);
    }
}
</script>

<style scoped>
#animation {
  display: inline-block;
  border: 1px solid gray;
  padding: 10px;
  width: 400px;
}
</style>
