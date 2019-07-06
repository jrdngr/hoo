<template>
  <div id="animation">
    <div class="control">
      <button @click="rotate">Rotate</button>
      <button @click="random">Random</button>
      <button @click="stop">Stop</button>
    </div>
    <div class="control">
      <input id="trans-time" type="number" min="0" max="65535" v-model="transitionTime" />
      <label for="trans-time">Transition time</label>
    </div>
    <div class="control">
      <input id="hold-time" type="number" min="0" max="65535" v-model="holdTime" />
      <label for="hold-time">Hold time</label>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import { BASE_URL } from '@/common/constants';
import * as AnimationApi from '@/common/api/animations';

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
        async rotate(event: any) {
            await AnimationApi.rotate(this.transitionTime, this.holdTime);
        },
        async random(event: any) {
            await AnimationApi.random(this.transitionTime, this.holdTime);
        },
        async stop(event: any) {
            await AnimationApi.stop();
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
