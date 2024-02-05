const baseUrl = "http://pi-grow.local:3000"

export type Sensors = {
    temperature: number
    humidity: number
}

export async function sensorState(): Promise<Sensors> {
    const response = await fetch(`${baseUrl}/sensors`)
    return response.json() as Promise<Sensors>
}