export class HooMotionSensor {
    public readonly name: string;
    public state: MotionState;

    constructor(name: string, state: MotionState) {
        this.name = name;
        this.state = state;
    }

    public triggered(): boolean {
        return this.state.presence;
    }
}

export interface MotionState {
    presence: boolean;
}
