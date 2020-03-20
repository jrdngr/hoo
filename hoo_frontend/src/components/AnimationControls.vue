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
    <div class="control">
        <input id="light-numbers" type="text" v-model="lightNumbers" />
        <label for="light-numbers">Light Numbers</label>
    </div>
    <div class="control">
        <button @click="addColor">Add Color</button>
        <ul id="color-list">
            <li class="colorPicker" v-for="(hue, index) in hues" :key="index">
                <input
                    type="range"
                    min="0"
                    max="65535"
                    @input="setColor($event, index)"
                />
                <svg class="colorPreview">
                    <circle cx="50%" cy="50%" r="20px" :fill="`hsl(${(hue / 65535) * 360}, 100%, 50%)`" />
                </svg>
            </li>
        </ul>
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
        const transitionTime: number = 5;
        const holdTime: number = 10;
        const lightNumbers: string = '';
        const hues: number[] = [];

        return {
            transitionTime,
            holdTime,
            lightNumbers,
            hues,
        };
    },
    methods: {
        async rotate(event: any) {
            await AnimationApi.rotate(this.transitionTime, this.holdTime, this.lightNumbers, this.hues);
        },
        async random(event: any) {
            await AnimationApi.random(this.transitionTime, this.holdTime, this.lightNumbers);
        },
        async stop(event: any) {
            await AnimationApi.stop();
        },
        addColor(event: any) {
            this.hues.push(0);
        },
        setColor(event: any, index: number) {
            this.hues.splice(index, 1, event.srcElement.value);
        }
    },
});
</script>

<style scoped>
#animation {
    display: inline-block;
    border: 1px solid gray;
    padding: 10px;
    width: 500px;
}

.colorPicker {
    display: flex;
}
</style>
