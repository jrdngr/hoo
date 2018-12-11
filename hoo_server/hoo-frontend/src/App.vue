<template>
  <div id="app">
    <LightControls v-bind:lightNumber="3"/>
    <LightControls v-bind:lightNumber="4"/>
  </div>
</template>

<script>
import LightControls from "./components/LightControls.vue";

export default {
  name: "app",
  components: {
    LightControls
  }
};
</script>

<style>
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
