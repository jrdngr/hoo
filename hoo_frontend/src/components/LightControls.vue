<template>
  <div id="light">
      <div id="header">
        <h1>{{lightNumber}}: {{lightName}}</h1>
        <svg id="preview">
            <circle cx="50%" cy="50%" r="20px" :fill='previewFillColor'/>
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
            <stop offset="0%" stop-color="hsl(0,100%,50%)"></stop>
            <stop offset="10%" stop-color="hsl(36,100%,50%)"></stop>
            <stop offset="20%" stop-color="hsl(72,100%,50%)"></stop>
            <stop offset="30%" stop-color="hsl(108,100%,50%)"></stop>
            <stop offset="40%" stop-color="hsl(144,100%,50%)"></stop>
            <stop offset="50%" stop-color="hsl(180,100%,50%)"></stop>
            <stop offset="60%" stop-color="hsl(216,100%,50%)"></stop>
            <stop offset="70%" stop-color="hsl(252,100%,50%)"></stop>
            <stop offset="80%" stop-color="hsl(288,100%,50%)"></stop>
            <stop offset="90%" stop-color="hsl(324,100%,50%)"></stop>
            <stop offset="100%" stop-color="hsl(360,100%,50%)"></stop>
          </linearGradient>
        </defs>
        <rect width="130" height="10" fill="url(#hue-gradient)"></rect>
      </svg>
      <input id="hue" type="range" min="0" max="65535" v-bind:value="hue" @input="set_hue">
      <label for="hue">Hue</label>
    </div>
    <div class="control">
      <input id="sat" type="range" min="0" max="255" v-bind:value="saturation" @input="set_sat">
      <label for="sat">Saturation</label>
    </div>
    <div class="control">
      <input id="bri" type="range" min="0" max="255" v-bind:value="brightness" @input="set_bri">
      <label for="bri">Brightness</label>
    </div>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { BASE_URL, INPUT_THROTTLING_DELAY } from "../Hoo.vue";

@Component
export default class LightControls extends Vue {
    @Prop({type: String, required: true}) private lightName!: string;
    @Prop({type: Number, required: true}) private lightNumber!: number;

    private hue: number = 0;
    private saturation: number = 0;
    private brightness: number = 0;

    private get previewFillColor(): string {
        const h = (this.hue / 65535) * 360;
        const s = (this.saturation / 255) * 100;
        const l = (this.brightness / 255) * 100;

        return `hsl(${h}, ${s}%, ${l}%)`;
    }

    private created() {
        const url = `${BASE_URL}/light/${this.lightNumber}`;
        fetch(url)
            .then(data => data.json())
            .then(light => {
                this.hue = light.state.hue;
                this.saturation = light.state.sat;
                this.brightness = light.state.bri;
            });
    }

    private on(event: any) {
      const url = `${BASE_URL}/${this.lightNumber}/on`;
      fetch(url);
    }

    private off(event: any) {
      const url = `${BASE_URL}/${this.lightNumber}/off`;
      fetch(url);
    }

    private set_bri(event: any) { 
        this.brightness = event.srcElement.value;

        const url = `${BASE_URL}/${this.lightNumber}/state?bri=${this.brightness}`;
        fetch(url);
    }

    private set_sat(event: any) { 
        this.saturation = event.srcElement.value;

        const url = `${BASE_URL}/${this.lightNumber}/state?sat=${this.saturation}`;
        fetch(url);
    }

    private set_hue(event: any) {
        this.hue = event.srcElement.value;

        const url = `${BASE_URL}/${this.lightNumber}/state?hue=${this.hue}`;
        fetch(url);
    }
}
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
