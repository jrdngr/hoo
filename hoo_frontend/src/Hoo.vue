<template>
  <div id="hoo">
    <animation-controls />
    <button @click="onAddFakeLightClicked">Add Fake Light</button>
    <ul>
      <li v-for="light in lights" :key="light.number">
        <LightControls v-bind:light="light" />
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import { Light, HooLight, FakeLight } from '@/common/types/light';
import LightControls from '@/components/LightControls.vue';
import AnimationControls from '@/components/AnimationControls.vue';
import * as LightApi from '@/common/api/lights';

export default Vue.extend({
    name: 'hoo',
    components: { LightControls, AnimationControls },
    data() {
        const lights: Light[] = [];
        const fakeLightNumber: number = 0;
        return {
            lights,
            fakeLightNumber,
        };
    },
    async created() {
        const lights = await LightApi.getAllLights();
        for (const lightNum in lights) {
            const lightNumber = parseInt(lightNum, 10);
            const light = lights[lightNum];
            this.lights.push(
                new HooLight(light.name, lightNumber, light.state),
            );
        }
    },
    methods: {
        onAddFakeLightClicked() {
            this.fakeLightNumber -= 1;
            const light = new FakeLight(
                `Fake Light ${this.fakeLightNumber}`,
                this.fakeLightNumber,
            );
            this.lights.push(light);
        },
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
