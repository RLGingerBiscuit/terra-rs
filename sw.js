var cacheName = "terra-rs";
var filesToCache = [
  "./",
  "./index.html",
  "./terra-rs.js",
  "./terra-rs_bg.wasm",
  "./resources/buffs.json",
  "./resources/buffs.png",
  "./resources/icons.png",
  "./resources/items.json",
  "./resources/items.png",
  "./resources/prefixes.json",
];

/* Start the service worker and cache all of the app's content */
self.addEventListener("install", function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    }),
  );
});

/* Serve cached content when offline */
self.addEventListener("fetch", function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    }),
  );
});
