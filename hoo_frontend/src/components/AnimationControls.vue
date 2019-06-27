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
import Vue from 'vue';
import { BASE_URL } from '@/common/constants';

export default Vue.extend({
    name: 'animationControls',
    data() {
        const transitionTime: number = 10;
        const holdTime: number = 0;

        return {
            transitionTime,
            holdTime,
        };
    },
    methods: {
        rotate(event: any) {
            const url = `${BASE_URL}/rotate/${this.transitionTime}/${
                this.holdTime
            }`;
            fetch(url);
        },
        random(event: any) {
            const url = `${BASE_URL}/random/${this.transitionTime}/${
                this.holdTime
            }`;
            fetch(url);
        },
        stop(event: any) {
            const url = `${BASE_URL}/stop`;
            fetch(url);
        },
    },
});
</script>

<style scoped>
#animation {
    display: inline-block;
    border: 1px solid gray;
    padding: 10px;
    width: 400px;
}
</style>
