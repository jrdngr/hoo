<template>
  <div id="app">
    <animation-controls/>
    <ul>
      <li v-for="light in lights" :key="light.number">
        <LightControls v-bind:lightNumber="light.number" v-bind:lightName="light.name"/>
      </li>
    </ul>
  </div>
</template>

<script>
import LightControls from "./components/LightControls.vue";
import AnimationControls from "./components/AnimationControls.vue";

export const BASE_URL = `http://${process.env.VUE_APP_IP}`;
export const INPUT_THROTTLING_DELAY = 100;

export default {
  name: "app",
  components: {
    LightControls,
    AnimationControls
  },
  data: function() {
    return {
      lights: []
    };
  },
  created: function() {
    const url = `${BASE_URL}/lights`;
    fetch(url)
      .then(data => data.json())
      .then(lights => {
        for (let lightNum in lights) {
          this.lights.push(new Light(lightNum, lights[lightNum].name));
        }
      });
  }
};

class Light {
  constructor(number, name) {
    this.number = parseInt(number);
    this.name = name;
  }
}
</script>

<style>
ul {
  list-style-type: none;
}

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
