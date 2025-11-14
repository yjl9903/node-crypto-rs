import { Crypto } from '@peculiar/webcrypto';
import { describe, bench } from 'vitest';

import { decryptAesGcm, encryptAesGcm } from '../index.js';

if (!global.crypto) {
  global.crypto = new Crypto();
}

describe('aes gcm', async () => {
  const key = await crypto.subtle.generateKey({ name: 'AES-GCM', length: 256 }, true, [
    'encrypt',
    'decrypt'
  ]);

  const keyBuffer = await crypto.subtle.exportKey('raw', key);

  const text = Buffer.from('hello world', 'utf-8');

  const n = 100;

  bench('node sync', async () => {
    const tasks = [];
    for (let i = 0; i < n; i++) {
      const fn = async () => {
        const iv = crypto.getRandomValues(new Uint8Array(12));
        const value = await crypto.subtle.encrypt({ name: 'AES-GCM', iv }, key, text);
        await crypto.subtle.decrypt({ name: 'AES-GCM', iv }, key, value);
      };
      tasks.push(await fn());
    }
  });

  bench('node parallel', async () => {
    const tasks = [];
    for (let i = 0; i < n; i++) {
      const fn = async () => {
        const iv = crypto.getRandomValues(new Uint8Array(12));
        const value = await crypto.subtle.encrypt({ name: 'AES-GCM', iv }, key, text);
        await crypto.subtle.decrypt({ name: 'AES-GCM', iv }, key, value);
      };
      tasks.push(fn());
    }
    await Promise.all(tasks);
  });

  bench('rust sync', async () => {
    const tasks = [];
    for (let i = 0; i < n; i++) {
      const fn = async () => {
        const result = await encryptAesGcm(Buffer.from(keyBuffer), text);
        await decryptAesGcm(Buffer.from(keyBuffer), result);
      };
      tasks.push(await fn());
    }
  });

  bench('rust parallel', async () => {
    const tasks = [];
    for (let i = 0; i < n; i++) {
      const fn = async () => {
        const result = await encryptAesGcm(Buffer.from(keyBuffer), text);
        await decryptAesGcm(Buffer.from(keyBuffer), result);
      };
      tasks.push(fn());
    }
    await Promise.all(tasks);
  });
});
