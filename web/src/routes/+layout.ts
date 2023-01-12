export let ssr = false;

export function load(event: any) {
    console.log('load', event);

    return {
        data: "test",
    }
}
