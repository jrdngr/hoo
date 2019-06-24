<template>
  <div id="hoo">
    <animation-controls/>
    <ul>
      <li v-for="light in lights" :key="light.number">
        <LightControls v-bind:lightNumber="light.number" v-bind:lightName="light.name"/>
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import LightControls from "./components/LightControls.vue";
import AnimationControls from "./components/AnimationControls.vue";

declare let process: any;

export const BASE_URL = `http://${process.env.VUE_APP_IP}`;
export const INPUT_THROTTLING_DELAY = 100;

@Component({
  components: { LightControls, AnimationControls }
})
export default class Hoo extends Vue {
  private lights: Light[] = [];

  private created() {
    const url = `${BASE_URL}/lights`;
    fetch(url)
      .then(data => data.json())
      .then(lights => {
        for (let lightNum in lights) {
          this.lights.push(new Light(lightNum, lights[lightNum].name));
        }
      });
  }
}

class Light {
  private number: number;
  private name: string;

  constructor(number: string, name: string) {
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
