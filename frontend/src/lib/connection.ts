import { browser } from '$app/environment';
import { get, writable } from 'svelte/store';
import { health } from '$lib/api';

export type ConnectionMode = 'normal' | 'low';

type ConnectionState = {
  mode: ConnectionMode;
  label: string;
  detail: string;
  latencyMs?: number;
  checkedAt?: number;
};

const normalDetail = 'Internet connection looks normal.';
const lowDetail =
  'Low connection mode. Cached pages and local records may be used when possible.';

export const connectionState = writable<ConnectionState>({
  mode: 'normal',
  label: 'Connection: Normal',
  detail: normalDetail
});

let monitorStarted = false;

function readBrowserSignal(): ConnectionMode {
  if (!browser) return 'normal';
  if (!navigator.onLine) return 'low';

  const connection = (
    navigator as Navigator & {
      connection?: { effectiveType?: string; saveData?: boolean };
    }
  ).connection;

  if (connection?.saveData) return 'low';
  if (connection?.effectiveType && ['slow-2g', '2g', '3g'].includes(connection.effectiveType)) {
    return 'low';
  }
  return 'normal';
}

function updateState(mode: ConnectionMode, latencyMs?: number) {
  connectionState.set({
    mode,
    label: mode === 'normal' ? 'Connection: Normal' : 'Connection: Low',
    detail: mode === 'normal' ? normalDetail : lowDetail,
    latencyMs,
    checkedAt: Date.now()
  });
}

export async function checkConnection() {
  if (!browser) return;

  const browserSignal = readBrowserSignal();
  if (browserSignal === 'low' && !navigator.onLine) {
    updateState('low');
    return;
  }

  const started = performance.now();
  const controller = new AbortController();
  const timeout = window.setTimeout(() => controller.abort(), 3000);

  try {
    await health(controller.signal);
    const latencyMs = Math.round(performance.now() - started);
    updateState(browserSignal === 'low' || latencyMs > 1200 ? 'low' : 'normal', latencyMs);
  } catch {
    updateState('low');
  } finally {
    window.clearTimeout(timeout);
  }
}

export function startConnectionMonitor() {
  if (!browser || monitorStarted) return;
  monitorStarted = true;

  updateState(readBrowserSignal());
  void checkConnection();

  const interval = window.setInterval(() => {
    void checkConnection();
  }, 45000);

  const refresh = () => void checkConnection();
  window.addEventListener('online', refresh);
  window.addEventListener('offline', refresh);

  return () => {
    window.clearInterval(interval);
    window.removeEventListener('online', refresh);
    window.removeEventListener('offline', refresh);
    monitorStarted = false;
  };
}

export function isLowConnection() {
  return get(connectionState).mode === 'low';
}
