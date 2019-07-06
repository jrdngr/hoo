import { BASE_URL } from '@/common/constants';
import Light from '@/common/types/light';

export async function getAllLights(): Promise<Light[]> {
    const url = `${BASE_URL}/lights`;
    const response = await fetch(url);
    const lights: Light[] = await response.json();
    return lights;
}

export async function getLight(lightNumber: number): Promise<Light> {
    const url = `${BASE_URL}/light/${lightNumber}`;
    const response: any = await fetch(url);
    const light: Light = await response.json();
    return light;
}

export async function on(lightNumber: number): Promise<void> {
    const url = `${BASE_URL}/${lightNumber}/on`;
    await fetch(url);
}

export async function off(lightNumber: number): Promise<void> {
    const url = `${BASE_URL}/${lightNumber}/off`;
    await fetch(url);
}

export async function setBrightness(lightNumber: number, brightness: number): Promise<void> {
    const url = `${BASE_URL}/${lightNumber}/state?bri=${brightness}`;
    await fetch(url);
}

export async function setSaturation(lightNumber: number, saturation: number): Promise<void> {
    const url = `${BASE_URL}/${lightNumber}/state?sat=${saturation}`;
    await fetch(url);
}

export async function setHue(lightNumber: number, hue: number): Promise<void> {
    const url = `${BASE_URL}/${lightNumber}/state?hue=${hue}`;
    await fetch(url);
}
