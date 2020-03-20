declare let process: any;

export const BASE_URL = `http://${process.env.VUE_APP_IP || 'localhost:8000'}/api`;
export const INPUT_THROTTLING_DELAY = 100;
