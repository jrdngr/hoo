export function shuffleArray<T>(array: T[]) {
    let n = array.length - 1;
    while (n > 1) {
        n -= 1;
        const k = randomInt(0, n);
        const temp = array[n];
        array[n] = array[k];
        array[k] = temp;
    }
}

export function randomInt(min: number, max: number): number {
    const range = max - min + 1;
    return Math.floor(Math.random() * range + min);
} 
