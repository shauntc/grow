export function repeat<T>(fn: () => Promise<T>, interval: number) {
    let timeout: number | undefined;
    const runner = async () => {
        await fn()
        timeout = setTimeout(runner, interval)
    }
    runner()

    return {
        cancel() {
            if (timeout != null) clearTimeout(timeout)
        }
    }
}