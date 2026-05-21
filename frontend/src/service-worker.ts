/// <reference lib="webworker" />

import { build, files, version } from '$service-worker';

const worker = self as unknown as ServiceWorkerGlobalScope;
const APP_CACHE = `gemma-herbalcare-app-${version}`;
const DATA_CACHE = `gemma-herbalcare-data-${version}`;
const APP_SHELL = ['/', '/consult', '/herbs', '/about'];
const APP_ASSETS = [...build, ...files, ...APP_SHELL];

worker.addEventListener('install', (event) => {
  event.waitUntil(
    caches
      .open(APP_CACHE)
      .then((cache) => cache.addAll(APP_ASSETS))
      .then(() => worker.skipWaiting())
  );
});

worker.addEventListener('activate', (event) => {
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys
            .filter((key) => key.startsWith('gemma-herbalcare-') && key !== APP_CACHE && key !== DATA_CACHE)
            .map((key) => caches.delete(key))
        )
      )
      .then(() => worker.clients.claim())
  );
});

worker.addEventListener('fetch', (event) => {
  const { request } = event;
  if (request.method !== 'GET') return;

  const url = new URL(request.url);
  if (url.origin !== worker.location.origin) return;

  if (request.mode === 'navigate') {
    event.respondWith(networkFirst(request, APP_CACHE, '/'));
    return;
  }

  if (url.pathname === '/api/demo-cases' || url.pathname === '/api/herbs') {
    event.respondWith(networkFirst(request, DATA_CACHE));
    return;
  }

  event.respondWith(cacheFirst(request, APP_CACHE));
});

async function cacheFirst(request: Request, cacheName: string) {
  const cached = await caches.match(request);
  if (cached) return cached;

  const response = await fetch(request);
  if (response.ok) {
    const cache = await caches.open(cacheName);
    await cache.put(request, response.clone());
  }
  return response;
}

async function networkFirst(request: Request, cacheName: string, fallbackPath?: string) {
  const cache = await caches.open(cacheName);
  try {
    const response = await fetch(request);
    if (response.ok) {
      await cache.put(request, response.clone());
    }
    return response;
  } catch {
    const cached = await cache.match(request);
    if (cached) return cached;
    if (fallbackPath) {
      const fallback = await caches.match(fallbackPath);
      if (fallback) return fallback;
    }
    throw new Error('No cached response available');
  }
}
