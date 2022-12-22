<template>
  <p>hi!</p>
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
          this.sensors = await LightApi.getAllMotionSensors();
        },
    },
});

export class MotionSensors extends Vue {}
</script>

<style scoped>

</style>
