import React from 'react';
import { BASE_URL, INPUT_THROTTLING_DELAY } from '../common/constants';
import { Light } from '../common/types/light';

const POLLING_DELAY_MS: number = 5000;

interface LightControlsProps {
    light: Light;
}

interface LightControlsState {
    light: Light;
}

export default class LightControls extends React.Component<LightControlsProps, LightControlsState> {
    constructor(props: LightControlsProps) {
        super(props);

        this.state = {
            light: props.light,
        }
    }

    render() {
        return(
            <div id="light">
            <div id="header">
              <h1>{this.state.light.number}: {this.state.light.name}</h1>
              <svg id="preview">
                <circle cx="50%" cy="50%" r="20px" fill={this.previewFillColor()} />
              </svg>
            </div>
            <div className="control">
              <button onClick={this.on}>On</button>
              <button onClick={this.off}>Off</button>
            </div>
            <div className="control">
              <svg id="hue-rainbow" width="130" height="10">
                <defs>
                  <linearGradient id="hue-gradient" x1="0%" y1="50%" x2="100%" y2="50%">
                    <stop offset="0%" stopColor="hsl(0,100%,50%)" />
                    <stop offset="10%" stopColor="hsl(36,100%,50%)" />
                    <stop offset="20%" stopColor="hsl(72,100%,50%)" />
                    <stop offset="30%" stopColor="hsl(108,100%,50%)" />
                    <stop offset="40%" stopColor="hsl(144,100%,50%)" />
                    <stop offset="50%" stopColor="hsl(180,100%,50%)" />
                    <stop offset="60%" stopColor="hsl(216,100%,50%)" />
                    <stop offset="70%" stopColor="hsl(252,100%,50%)" />
                    <stop offset="80%" stopColor="hsl(288,100%,50%)" />
                    <stop offset="90%" stopColor="hsl(324,100%,50%)" />
                    <stop offset="100%" stopColor="hsl(360,100%,50%)" />
                  </linearGradient>
                </defs>
                <rect width="130" height="10" fill="url(#hue-gradient)" />
              </svg>
              <input
                id="hue"
                type="range"
                min="0"
                max="65535"
                // value={this.state.light.hue}
                onChange={this.setHue}
              />
              <label htmlFor="hue">Hue</label>
            </div>
            <div className="control">
              <input
                id="sat"
                type="range"
                min="0"
                max="255"
                // value={this.state.light.saturation}
                onChange={this.setSat}
              />
              <label htmlFor="sat">Saturation</label>
            </div>
            <div className="control">
              <input
                id="bri"
                type="range"
                min="0"
                max="255"
                // value={this.state.light.brightness}
                onChange={this.setBri}
              />
              <label htmlFor="bri">Brightness</label>
            </div>
          </div>
        );
    }

    previewFillColor(): string {
        const h = (this.state.light.hue / 65535) * 360;
        const s = (this.state.light.saturation / 255) * 100;
        const l = (this.state.light.brightness / 255) * 100;

        return `hsl(${h}, ${s}%, ${l}%)`;
    }

    async componentDidMount() {
        await this.updateState();
    }

    on = async () => {
        await this.state.light.on();
    }

    off = async () => {
        await this.state.light.off();
    }

    setBri = async (event: any) => {
        await this.state.light.setBrightness(event.target.value);
    }

    setSat = async (event: any) => {
        await this.state.light.setSaturation(event.target.value);
    }

    setHue = async (event: any) => {
        await this.state.light.setHue(event.target.value);
    }

    updateState = async () => {
        await this.state.light.update();
    }
}
