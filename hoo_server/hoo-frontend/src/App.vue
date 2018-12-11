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

export const INPUT_THROTTLING_DELAY = 100;

export default {
  name: "app",
  components: {
    LightControls,
    AnimationControls
  }
};
</script>

<style>
button {
  font-size: 1.5em;
  margin: 10px;
}

input {
  font-size: 1.5em;
}

label {
  font-size: 1.5em;
  margin-left: 10px;
}

.control {
  margin: 10px;
}
</style>
