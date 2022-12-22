import * as LightApi from "@/common/api/lights";
import { randomInt } from "@/common/utils/random";

export interface Motion {
    readonly name: string;
    readonly presence: boolean;

    update(): Promise<void>;
}

export class HooMotion implements Motion {
    public readonly name: string;
    public state: MotionState;

    constructor(name: string, state: MotionState) {
        this.name = name;
        this.state = state;
    }

    public async update() {
        let sensors = await LightApi.getAllMotionSensors();
        this.state = updatedLight.state;
    }
}

export interface MotionState {
    presence: boolean;
}
