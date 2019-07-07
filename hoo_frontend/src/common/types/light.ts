import * as LightApi from "@/common/api/lights";

export interface Light {
    readonly name: string;
    readonly number: number;
    state: LightState;

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

    public async update(): Promise<void> {
        let updatedLight = await LightApi.getLight(this.number);
        this.state = updatedLight.state;
    }

    public async on(): Promise<void> {
        this.state.on = true;
        await LightApi.on(this.number);
    }

    public async off(): Promise<void> {
        this.state.on = false;
        await LightApi.off(this.number);
    }

    public async setHue(value: number): Promise<void> {
        this.state.hue = value;
        await LightApi.setHue(this.number, value);
    }

    public async setSaturation(value: number): Promise<void> {
        this.state.sat = value;
        await LightApi.setSaturation(this.number, value);
    }

    public async setBrightness(value: number): Promise<void> {
        this.state.bri = value;
        await LightApi.setBrightness(this.number, value);
    }
}

export class FakeLight implements Light {
    public readonly name: string;
    public readonly number: number;
    public state: LightState;

    constructor(name: string, lightNumber: number, state: LightState) {
        this.number = lightNumber;
        this.name = name;
        this.state = state;
    }

    public async update(): Promise<void> {
        // It's already updated!
    }

    public async on(): Promise<void> {
        this.state.on = true;
    }

    public async off(): Promise<void> {
        this.state.on = false;
    }

    public async setHue(value: number): Promise<void> {
        this.state.hue = value;
    }

    public async setSaturation(value: number): Promise<void> {
        this.state.sat = value;
    }

    public async setBrightness(value: number): Promise<void> {
        this.state.bri = value;
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
