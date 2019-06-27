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
import Vue from 'vue';
import { BASE_URL } from '@/common/constants';
import Light from '@/common/types/light';
import LightControls from '@/components/LightControls.vue';
import AnimationControls from '@/components/AnimationControls.vue';

export default Vue.extend({
    name: 'hoo',
    components: { LightControls, AnimationControls },
    data() {
        const lights: Light[] = [];
        return {
            lights,
        };
    },
    async created() {
        const url = `${BASE_URL}/lights`;
        const response = await fetch(url);
        const lights: Light[] = await response.json();
        for (const lightNum in lights) {
            const lightNumber = parseInt(lightNum, 10);
            const light = lights[lightNum];
            this.lights.push(new Light(light.name, lightNumber, light.state));
        }
    },
});
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
