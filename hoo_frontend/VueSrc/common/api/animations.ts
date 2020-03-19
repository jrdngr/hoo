import { BASE_URL } from '@/common/constants';

export async function rotate(transitionTime: number, holdTime: number): Promise<void> {
    const url = `${BASE_URL}/rotate/${transitionTime}/${holdTime}`;
    await fetch(url);
}

export async function random(transitionTime: number, holdTime: number): Promise<void> {
    const url = `${BASE_URL}/random/${transitionTime}/${holdTime}`;
    await fetch(url);
}

export async function stop(): Promise<void> {
    const url = `${BASE_URL}/stop`;
    await fetch(url);
}
