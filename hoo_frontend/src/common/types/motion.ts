export class HooMotionSensor {
    public readonly name: string;
    public state: MotionState;

    constructor(name: string, state: MotionState) {
        this.name = name;
        this.state = state;
    }
}

export interface MotionState {
    presence: boolean;
}
