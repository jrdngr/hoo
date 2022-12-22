<template>
  <ul>
    <li v-for="sensor in sensors" :key="sensor.name">
      <h1 :class="{ red: sensor.state.presence}">{{ sensor.name }}</h1>
    </li>
  </ul>
</template>

<script lang="ts">
import Vue from 'vue';
import { BASE_URL, INPUT_THROTTLING_DELAY } from '@/common/constants';
import * as LightApi from '@/common/api/lights';
import { HooMotionSensor } from '@/common/types/motion';

const POLLING_DELAY_MS: number = 5000;

export default Vue.extend({
    name: 'motionSensors',
    data() {
      const sensors: HooMotionSensor[] = [];
        return {
            sensors,
        };
    },
    async created() {
        this.updateState();
    },
    methods: {
        async updateState() {
          while (this.sensors.length > 0) {
            this.sensors.pop();
          }
          const sensors = await LightApi.getAllMotionSensors();
          sensors.forEach(sensor => this.sensors.push(sensor))
          console.log(this.sensors);

          await setTimeout(() => this.updateState(), POLLING_DELAY_MS);
        },
    },
});

export class MotionSensors extends Vue {}
</script>

<style scoped>
.red {
  color: red;
}

h1 {
  color: black;
}
</style>
