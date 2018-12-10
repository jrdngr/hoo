<template>
  <div id="light">
    <h1>I am a light!</h1>
    <button v-on:click="on">On</button>
    <button v-on:click="off">Off</button>
  </div>
</template>

<script>
export default {
  name: "LightControls",
  data: function() {
    return {
      lightNum: 1
    };
  },
  methods: {
    on: function(event) {
      const url = "http://localhost:8080/on/4";
      fetch(url)
        .then(data => data.json())
        .then(res => console.log(res));
    },
    off: function(event) {
      const url = "http://localhost:8080/off/4";
      fetch(url)
        .then(data => data.json())
        .then(res => console.log(res));
    }
  }
};
</script>

<style>
</style>
