import * as LightApi from "../api/lights";
import { randomInt } from "../utils/random";

export interface Light {
    readonly name: string;
    readonly number: number;

    readonly isOn: boolean;
    readonly hue: number;
    readonly saturation: number;
    readonly brightness: number;

    update(): Promise<void>;
    on(): Promise<void>;
    off(): Promise<void>;
    setHue(value: number): Promise<void>;
    setSaturation(value: number): Promise<void>;
    setBrightness(value: number): Promise<void>;
}

export class HooLight implements Light {
    public readonly name: string;
    public readonly number: number;
    public state: LightState;

    constructor(name: string, lightNumber: number, state: LightState) {
        this.number = lightNumber;
        this.name = name;
        this.state = state;
    }

    public async update() {
        let updatedLight = await LightApi.getLight(this.number);
        this.state = updatedLight.state;
    }

    public get isOn(): boolean {
        return this.state.on;
    }

    public get hue(): number {
        return this.state.hue;
    }

    public get saturation(): number {
        return this.state.sat;
    }

    public get brightness(): number {
        return this.state.bri;
    }

    public async on() {
        this.state.on = true;
        await LightApi.on(this.number);
    }

    public async off() {
        this.state.on = false;
        await LightApi.off(this.number);
    }

    public async setHue(value: number) {
        this.state.hue = value;
        await LightApi.setHue(this.number, value);
    }

    public async setSaturation(value: number) {
        this.state.sat = value;
        await LightApi.setSaturation(this.number, value);
    }

    public async setBrightness(value: number) {
        this.state.bri = value;
        await LightApi.setBrightness(this.number, value);
    }
}

export class FakeLight implements Light {
    public readonly name: string;
    public readonly number: number;

    private _isOn: boolean;
    private _hue: number;
    private _saturation: number;
    private _brightness: number;

    constructor(name: string, lightNumber: number) {
        this.number = lightNumber;
        this.name = name;
        this._isOn = false;
        this._hue = randomInt(0, 64435);
        this._saturation = randomInt(0, 255);
        this._brightness = randomInt(0, 255);
    }

    public async update() {
        // It's already updated!
    }

    public get isOn(): boolean {
        return this._isOn;
    }

    public get hue(): number {
        return this._hue;
    }

    public get saturation(): number {
        return this._saturation;
    }

    public get brightness(): number {
        return this._brightness;
    }

    public async on() {
        this._isOn = true;
    }

    public async off() {
        this._isOn = false;
    }

    public async setHue(value: number) {
        this._hue = value;
    }

    public async setSaturation(value: number) {
        this._saturation = value;
    }

    public async setBrightness(value: number) {
        this._brightness = value;
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
