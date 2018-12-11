<template>
  <div id="app">
    <animation-controls/>
    <br>
    <LightControls v-bind:lightNumber="1"/>
    <br>
    <LightControls v-bind:lightNumber="2"/>
    <br>
    <LightControls v-bind:lightNumber="3"/>
    <br>
    <LightControls v-bind:lightNumber="4"/>
  </div>
</template>

<script>
import LightControls from "./components/LightControls.vue";
import AnimationControls from "./components/AnimationControls.vue";

export default {
  name: "app",
  components: {
    LightControls,
    AnimationControls
  }
};
</script>

<style>
</style>
