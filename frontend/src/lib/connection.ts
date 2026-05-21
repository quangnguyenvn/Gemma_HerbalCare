import { browser } from '$app/environment';
import { get, writable } from 'svelte/store';
import { health } from '$lib/api';

export type ConnectionMode = 'normal' | 'low' | 'offline';

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
const offlineDetail =
  'Offline mode. You can still use cached pages, saved drafts, and previously saved regional records.';

export const connectionState = writable<ConnectionState>({
  mode: 'normal',
  label: 'Connection: Normal',
  detail: normalDetail
});

let monitorStarted = false;

function readBrowserSignal(): ConnectionMode {
  if (!browser) return 'normal';
  if (!navigator.onLine) return 'offline';

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
  const copy = {
    normal: {
      label: 'Connection: Normal',
      detail: normalDetail
    },
    low: {
      label: 'Connection: Low',
      detail: lowDetail
    },
    offline: {
      label: 'Connection: Offline',
      detail: offlineDetail
    }
  } satisfies Record<ConnectionMode, { label: string; detail: string }>;

  connectionState.set({
    mode,
    label: copy[mode].label,
    detail: copy[mode].detail,
    latencyMs,
    checkedAt: Date.now()
  });
}

export async function checkConnection() {
  if (!browser) return;

  const browserSignal = readBrowserSignal();
  if (browserSignal === 'offline') {
    updateState('offline');
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
    updateState(navigator.onLine ? 'low' : 'offline');
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
  const mode = get(connectionState).mode;
  return mode === 'low' || mode === 'offline';
}

export function isOfflineConnection() {
  return get(connectionState).mode === 'offline';
}
