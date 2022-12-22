import { BASE_URL } from '@/common/constants';
import { Light, HooLight } from '@/common/types/light';
import { HooMotionSensor } from '../types/motion';

export async function getAllLights(): Promise<HooLight[]> {
    const url = `${BASE_URL}/lights`;
    const response = await fetch(url);
    const lights: HooLight[] = await response.json();
    return lights;
}

export async function getLight(lightNumber: number): Promise<HooLight> {
    const url = `${BASE_URL}/light/${lightNumber}`;
    const response: any = await fetch(url);
    const light: HooLight = await response.json();
    return light;
}

export async function getAllMotionSensors(): Promise<HooMotionSensor[]> {
    const url = `${BASE_URL}/motion`;
    const response = await fetch(url);
    const lights: HooMotionSensor[] = await response.json();
    return lights;
}

export async function on(lightNumber: number) {
    const url = `${BASE_URL}/${lightNumber}/on`;
    await fetch(url);
}

export async function off(lightNumber: number) {
    const url = `${BASE_URL}/${lightNumber}/off`;
    await fetch(url);
}

export async function setBrightness(lightNumber: number, brightness: number) {
    const url = `${BASE_URL}/${lightNumber}/state?bri=${brightness}`;
    await fetch(url);
}

export async function setSaturation(lightNumber: number, saturation: number) {
    const url = `${BASE_URL}/${lightNumber}/state?sat=${saturation}`;
    await fetch(url);
}

export async function setHue(lightNumber: number, hue: number) {
    const url = `${BASE_URL}/${lightNumber}/state?hue=${hue}`;
    await fetch(url);
}
