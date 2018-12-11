<template>
  <div id="animation">
    <button v-on:click="rotate">Rotate</button>
    <button v-on:click="random">Random</button>
    <button v-on:click="stop">Stop</button>
    <br>
    <br>
    <input id="trans-time" type="number" min="0" max="65535" v-model="transitionTime">
    <label for="trans-time">Transition time</label>
    <br>
    <br>
    <input id="hold-time" type="number" min="0" max="65535" v-model="holdTime">
    <label for="hold-time">Hold time</label>
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
  width: 300px;
}

button {
  font-size: 1.5em;
}

input {
  font-size: 1.5em;
}

label {
  font-size: 1.5em;
  margin-left: 10px;
}
</style>
