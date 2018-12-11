<template>
  <div id="light">
    <h1>Light #{{lightNumber}}</h1>
    <button v-on:click="on">On</button>
    <button v-on:click="off">Off</button>
  </div>
</template>

<script>
export default {
  name: "LightControls",
  props: {
    lightNumber: Number
  },
  methods: {
    on: function(event) {
      const url = `http://localhost:8080/on/${this.lightNumber}`;
      fetch(url);
    },
    off: function(event) {
      const url = `http://localhost:8080/off/${this.lightNumber}`;
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
}

button {
  font-size: 1.5em;
}
</style>
