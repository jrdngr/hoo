<template>
  <div id="light">
    <h1>Light #{{lightNumber}}</h1>
    <button v-on:click="on">On</button>
    <button v-on:click="off">Off</button>
    <br>
    <br>
    <label for="hue">Hue</label>
    <br>
    <input id="hue" type="range" min="0" max="65535" v-on:change="hue">
    <br>
    <br>
    <label for="sat">Saturation</label>
    <br>
    <input id="sat" type="range" min="0" max="255" v-on:change="sat">
    <br>
    <br>
    <label for="bri">Brightness</label>
    <br>
    <input id="bri" type="range" min="0" max="255" v-on:change="bri">
  </div>
</template>

<script>
const baseUrl = "http://localhost:8080";

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
    bri: function(event) {
      const url = `${baseUrl}/state/${this.lightNumber}?bri=${
        event.srcElement.value
      }`;
      fetch(url);
    },
    sat: function(event) {
      const url = `${baseUrl}/state/${this.lightNumber}?sat=${
        event.srcElement.value
      }`;
      fetch(url);
    },
    hue: function(event) {
      const url = `${baseUrl}/state/${this.lightNumber}?hue=${
        event.srcElement.value
      }`;
      fetch(url);
    }
  }
};
</script>

<style scoped>
#light {
  display: inline-block;
  border: 1px solid gray;
  padding: 10px;
  width: 200px;
}

button {
  font-size: 1.5em;
}
</style>
