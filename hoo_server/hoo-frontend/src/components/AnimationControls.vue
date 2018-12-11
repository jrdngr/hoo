<template>
  <div id="animation">
    <div class="control">
      <button @click="rotate">Rotate</button>
      <button @click="random">Random</button>
      <button @click="stop">Stop</button>
    </div>
    <div class="control">
      <input id="trans-time" type="number" min="0" max="65535" v-model="transitionTime">
      <label for="trans-time">Transition time</label>
    </div>
    <div class="control">
      <input id="hold-time" type="number" min="0" max="65535" v-model="holdTime">
      <label for="hold-time">Hold time</label>
    </div>
  </div>
</template>

<script>
const baseUrl = `http://${process.env.VUE_APP_IP}`;

export default {
  name: "AnimationControls",
  data: function() {
    return {
      transitionTime: 10,
      holdTime: 0
    };
  },
  methods: {
    rotate: function(event) {
      const url = `${baseUrl}/rotate/${this.transitionTime}/${this.holdTime}`;
      fetch(url);
    },
    random: function(event) {
      const url = `${baseUrl}/random/${this.transitionTime}/${this.holdTime}`;
      fetch(url);
    },
    stop: function(event) {
      const url = `${baseUrl}/stop`;
      fetch(url);
    }
  }
};
</script>

<style scoped>
#animation {
  display: inline-block;
  border: 1px solid gray;
  padding: 10px;
  width: 400px;
}
</style>
