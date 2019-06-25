<template>
  <div id="light">
    <h1>{{lightNumber}}: {{lightName}}</h1>
    <div class="control">
      <button @click="on">On</button>
      <button @click="off">Off</button>
    </div>
    <div class="control">
      <svg id="hue-rainbow" width="130" height="10">
        <defs>
          <linearGradient id="hue-gradient" x1="0%" y1="50%" x2="100%" y2="50%">
            <stop offset="0%" stop-color="hsl(0,100%,50%)"></stop>
            <stop offset="10%" stop-color="hsl(36,100%,50%)"></stop>
            <stop offset="20%" stop-color="hsl(72,100%,50%)"></stop>
            <stop offset="30%" stop-color="hsl(108,100%,50%)"></stop>
            <stop offset="40%" stop-color="hsl(144,100%,50%)"></stop>
            <stop offset="50%" stop-color="hsl(180,100%,50%)"></stop>
            <stop offset="60%" stop-color="hsl(216,100%,50%)"></stop>
            <stop offset="70%" stop-color="hsl(252,100%,50%)"></stop>
            <stop offset="80%" stop-color="hsl(288,100%,50%)"></stop>
            <stop offset="90%" stop-color="hsl(324,100%,50%)"></stop>
            <stop offset="100%" stop-color="hsl(360,100%,50%)"></stop>
          </linearGradient>
        </defs>
        <rect width="130" height="10" fill="url(#hue-gradient)"></rect>
      </svg>
      <input id="hue" type="range" min="0" max="65535" v-bind:value="hue" @input="set_hue">
      <label for="hue">Hue</label>
    </div>
    <div class="control">
      <input id="sat" type="range" min="0" max="255" v-bind:value="saturation" @input="set_sat">
      <label for="sat">Saturation</label>
    </div>
    <div class="control">
      <input id="bri" type="range" min="0" max="255" v-bind:value="brightness" @input="set_bri">
      <label for="bri">Brightness</label>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import _ from "lodash";
import { BASE_URL, INPUT_THROTTLING_DELAY } from "../Hoo.vue";

@Component
export default class LightControls extends Vue {
    @Prop({type: String, required: true}) private lightName!: string;
    @Prop({type: Number, required: true}) private lightNumber!: number;

    private hue: number = 0;
    private saturation: number = 0;
    private brightness: number = 0;

  private created() {
    const url = `${BASE_URL}/light/${this.lightNumber}`;
    fetch(url)
      .then(data => data.json())
      .then(light => {
        this.hue = light.state.hue;
        this.saturation = light.state.sat;
        this.brightness = light.state.bri;
      });
  }
    private on(event: any) {
      const url = `${BASE_URL}/${this.lightNumber}/on`;
      fetch(url);
    }

    private off(event: any) {
      const url = `${BASE_URL}/${this.lightNumber}/off`;
      fetch(url);
    }

    private set_bri(event: any) { 
        _.throttle(function(this: LightControls, event: any) {
            const url = `${BASE_URL}/${this.lightNumber}/state?bri=${event.srcElement.value}`;
            fetch(url);
        }, INPUT_THROTTLING_DELAY);
    }

    private set_sat(event: any) { 
        _.throttle(function(this: LightControls, event: any) {
            const url = `${BASE_URL}/${this.lightNumber}/state?sat=${event.srcElement.value}`;
            fetch(url);
        }, INPUT_THROTTLING_DELAY);
    }

    private set_hue(event: any) {
         _.throttle(function(this: LightControls, event: any) {
            const url = `${BASE_URL}/${this.lightNumber}/state?hue=${event.srcElement.value}`;
            fetch(url);
        }, INPUT_THROTTLING_DELAY);
    }
}
</script>

<style scoped>
#light {
  display: inline-block;
  border: 1px solid gray;
  padding: 10px;
  width: 400px;
}

#hue-rainbow {
  display: block;
}
</style>
