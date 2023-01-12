import {writable} from 'svelte/store';

export const accessTokenStore = writable(localStorage.getItem('accessToken'));

export const refreshTokenStore = writable(localStorage.getItem('refreshToken'));

export const loggedIn$ = writable<string | null>(null);

accessTokenStore.subscribe(newValue => {
    if (newValue) localStorage.setItem('accessToken', newValue);
});

refreshTokenStore.subscribe(newValue => {
    if (newValue) localStorage.setItem('refreshToken', newValue);
});
