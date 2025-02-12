import { describe, bench } from 'vitest';

import { decrypt_aes_gcm, encrypt_aes_gcm } from '../index.js';

describe('aes gcm', async () => {
  const key = await crypto.subtle.generateKey({ name: 'AES-GCM', length: 256 }, true, [
    'encrypt',
    'decrypt'
  ]);

  const keyBuffer = await crypto.subtle.exportKey('raw', key);

  const text = Buffer.from('hello world', 'utf-8');

  bench('node', async () => {});

  bench('rust', async () => {
    const result = await encrypt_aes_gcm(Buffer.from(keyBuffer), text);
    await decrypt_aes_gcm(Buffer.from(keyBuffer), result);
  });
});
