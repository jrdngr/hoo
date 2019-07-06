<template>
  <div id="light">
    <div id="header">
      <h1>{{lightNumber}}: {{lightName}}</h1>
      <svg id="preview">
        <circle cx="50%" cy="50%" r="20px" :fill="previewFillColor" />
      </svg>
    </div>
    <div class="control">
      <button @click="on">On</button>
      <button @click="off">Off</button>
    </div>
    <div class="control">
      <svg id="hue-rainbow" width="130" height="10">
        <defs>
          <linearGradient id="hue-gradient" x1="0%" y1="50%" x2="100%" y2="50%">
            <stop offset="0%" stop-color="hsl(0,100%,50%)" />
            <stop offset="10%" stop-color="hsl(36,100%,50%)" />
            <stop offset="20%" stop-color="hsl(72,100%,50%)" />
            <stop offset="30%" stop-color="hsl(108,100%,50%)" />
            <stop offset="40%" stop-color="hsl(144,100%,50%)" />
            <stop offset="50%" stop-color="hsl(180,100%,50%)" />
            <stop offset="60%" stop-color="hsl(216,100%,50%)" />
            <stop offset="70%" stop-color="hsl(252,100%,50%)" />
            <stop offset="80%" stop-color="hsl(288,100%,50%)" />
            <stop offset="90%" stop-color="hsl(324,100%,50%)" />
            <stop offset="100%" stop-color="hsl(360,100%,50%)" />
          </linearGradient>
        </defs>
        <rect width="130" height="10" fill="url(#hue-gradient)" />
      </svg>
      <input id="hue" type="range" min="0" max="65535" v-bind:value="hue" @input="setHue" />
      <label for="hue">Hue</label>
    </div>
    <div class="control">
      <input id="sat" type="range" min="0" max="255" v-bind:value="saturation" @input="setSat" />
      <label for="sat">Saturation</label>
    </div>
    <div class="control">
      <input id="bri" type="range" min="0" max="255" v-bind:value="brightness" @input="setBri" />
      <label for="bri">Brightness</label>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import { BASE_URL, INPUT_THROTTLING_DELAY } from '@/common/constants';
import Light from '@/common/types/light';
import * as LightApi from '@/common/api/lights';

const POLLING_DELAY_MS: number = 5000;

export default Vue.extend({
    name: 'lightControls',
    props: {
        lightName: {
            type: String,
            required: true,
        },
        lightNumber: {
            type: Number,
            required: true,
        },
    },
    data() {
        const hue: number = 0;
        const saturation: number = 0;
        const brightness: number = 0;

        return {
            hue,
            saturation,
            brightness,
        };
    },
    computed: {
        previewFillColor(): string {
            const h = (this.hue / 65535) * 360;
            const s = (this.saturation / 255) * 100;
            const l = (this.brightness / 255) * 100;

            return `hsl(${h}, ${s}%, ${l}%)`;
        },
    },
    async created() {
        this.updateState();
    },
    methods: {
        async on(event: any) {
            await LightApi.on(this.lightNumber);
        },

        async off(event: any) {
            await LightApi.off(this.lightNumber);
        },

        async setBri(event: any) {
            this.brightness = event.srcElement.value;
            await LightApi.setBrightness(this.lightNumber, this.brightness);
        },

        async setSat(event: any) {
            this.saturation = event.srcElement.value;
            await LightApi.setSaturation(this.lightNumber, this.saturation);
        },

        async setHue(event: any) {
            this.hue = event.srcElement.value;
            await LightApi.setHue(this.lightNumber, this.hue);
        },
        async updateState() {
            const light = await LightApi.getLight(this.lightNumber);
            this.hue = light.state.hue;
            this.saturation = light.state.sat;
            this.brightness = light.state.bri;
        },
    },
});

export class LightControls extends Vue {}
</script>

<style scoped>
#header {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    align-items: center;
}

#header svg {
    width: 30%;
    height: 50px;
}

#light {
    display: inline-block;
    border: 1px solid gray;
    padding: 10px;
    width: 400px;
}

#hue-rainbow {
    display: block;
}
</style>
