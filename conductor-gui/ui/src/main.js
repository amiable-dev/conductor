import { mount } from 'svelte';
import App from './App.svelte';

console.log('main.js loaded!');
console.log('Target element:', document.getElementById('app'));

const app = mount(App, {
  target: document.getElementById('app'),
});

console.log('App instance mounted:', app);

export default app;
