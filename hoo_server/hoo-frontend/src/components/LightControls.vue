<template>
  <div id="light">
    <h1>Light #{{lightNumber}}</h1>
    <div class="control">
      <button @click="on">On</button>
      <button @click="off">Off</button>
    </div>
    <div class="control">
      <input id="hue" type="range" min="0" max="65535" @input="hue">
      <label for="hue">Hue</label>
    </div>
    <div class="control">
      <input id="sat" type="range" min="0" max="255" @input="sat">
      <label for="sat">Saturation</label>
    </div>
    <div class="control">
      <input id="bri" type="range" min="0" max="255" @input="bri">
      <label for="bri">Brightness</label>
    </div>
  </div>
</template>

<script>
import _ from "lodash";
import { INPUT_THROTTLING_DELAY } from "../App.vue";

const baseUrl = `http://${process.env.VUE_APP_IP}`;

export default {
  name: "LightControls",
  props: {
    lightNumber: Number
  },
  methods: {
    on: function(event) {
      const url = `${baseUrl}/on/${this.lightNumber}`;
      fetch(url);
    },
    off: function(event) {
      const url = `${baseUrl}/off/${this.lightNumber}`;
      fetch(url);
    },
    bri: _.throttle(function(event) {
      const url = `${baseUrl}/state/${this.lightNumber}?bri=${
        event.srcElement.value
      }`;
      fetch(url);
    }, INPUT_THROTTLING_DELAY),
    sat: _.throttle(function(event) {
      const url = `${baseUrl}/state/${this.lightNumber}?sat=${
        event.srcElement.value
      }`;
      fetch(url);
    }, INPUT_THROTTLING_DELAY),
    hue: _.throttle(function(event) {
      const url = `${baseUrl}/state/${this.lightNumber}?hue=${
        event.srcElement.value
      }`;
      fetch(url);
    }, INPUT_THROTTLING_DELAY)
  }
};
</script>

<style scoped>
#light {
  display: inline-block;
  border: 1px solid gray;
  padding: 10px;
  width: 400px;
}
</style>
