import React from 'react';
import * as AnimationApi from '../common/api/animations';

interface AnimationControlState {
    transitionTime: number ;
    holdTime: number;
}

export default class AnimationControls extends React.Component<{}, AnimationControlState> {
    constructor(props: {}) {
        super(props)

        this.state = {
            transitionTime: 10,
            holdTime: 0,
        }
    }

    render() {
        return(
            <div id="AnimationControls">
                <div className="control">
                    <button onClick={this.rotate}>Rotate</button>
                    <button onClick={this.random}>Random</button>
                    <button onClick={this.stop}>Stop</button>
                </div>
                <div className="control">
                    <input id="trans-time" type="number" min="0" max="65535" v-model="transitionTime" />
                    <label htmlFor="trans-time">Transition time</label>
                </div>
                <div className="control">
                    <input id="hold-time" type="number" min="0" max="65535" v-model="holdTime" />
                    <label htmlFor="hold-time">Hold time</label>
                </div>
            </div>
        );
    }

    async rotate() {
        await AnimationApi.rotate(this.state.transitionTime, this.state.holdTime);
    }
    
    async random() {
        await AnimationApi.random(this.state.transitionTime, this.state.holdTime);
    }
    
    async stop() {
        await AnimationApi.stop();
    }
}
