import Vue from 'vue';
import Hoo from './Hoo.vue';

Vue.config.productionTip = false;

new Vue({
    render: (h) => h(Hoo),
}).$mount('#app');
