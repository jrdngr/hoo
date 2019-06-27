export default class Light {
    public readonly name: string;
    public readonly lightNumber: number;
    public readonly state: LightState;

    constructor(name: string, lightNumber: number, state: LightState) {
        this.lightNumber = lightNumber;
        this.name = name;
        this.state = state;
    }
}

export interface LightState {
    on: boolean;
    hue: number;
    sat: number;
    bri: number;
    xy: [number, number];
    ct: number;
    effect: string;
    transitionTime: number;
    hue_inc: number;
    sat_inc: number;
    bri_inc: number;
    ct_inc: number;
    xy_inc: [number, number];
    reachable: boolean;
}
