const CACHE = 'stick-the-charge-v2';
const FILES = ['./', './index.html', './styles.css', './app.mjs', './photos.mjs', './pkg/stick_the_charge.js', './pkg/stick_the_charge_bg.wasm', './manifest.webmanifest', './icon.svg', './icon-192.png', './icon-512.png'];
self.addEventListener('install', event => event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(FILES))));
self.addEventListener('activate', event => event.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(key => key.startsWith('stick-the-charge-') && key !== CACHE).map(key => caches.delete(key))))));
self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET' || new URL(event.request.url).origin !== self.location.origin) return;
  const allowed = FILES.some(file => new URL(file, self.registration.scope).href === event.request.url);
  if (!allowed) return;
  event.respondWith(fetch(event.request).then(async response => {
    if (response.ok) await (await caches.open(CACHE)).put(event.request, response.clone());
    return response;
  }).catch(() => caches.match(event.request)));
});
