import React from 'react';
import { Light, HooLight, FakeLight } from './common/types/light';
import LightControls from './components/LightControls';
import AnimationControls from './components/AnimationControls';
import * as LightApi from './common/api/lights';
import './Hoo.css';

interface HooState {
    lights: Light[];
}

export default class Hoo extends React.Component<{}, HooState> {
    constructor(props: {}) {
        super(props);

        this.state = {
            lights: [],
        }
    }

    async componentDidMount() {
        const lights = await LightApi.getAllLights();
        const lightStates = [];
        for (const lightNum in lights) {
            const lightNumber = parseInt(lightNum, 10);
            const light = lights[lightNum];

            lightStates.push(
                new HooLight(light.name, lightNumber, light.state),
            );
        }

        this.setState({lights: lightStates});
    }

    render() {
        const lightControls = this.state.lights.map(light =>
            <li key={light.number}>
                <LightControls light={light} />
            </li>
        );

        return (
            <div id="Hoo">
                <AnimationControls />
                <ul>{lightControls}</ul>
            </div>
          );
    }
}
