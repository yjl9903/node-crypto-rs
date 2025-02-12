import { describe, it, expect } from 'vitest';

import { decrypt_aes_gcm, encrypt_aes_gcm } from '../index.js';

describe('aes gcm', async () => {
  const key = await crypto.subtle.generateKey({ name: 'AES-GCM', length: 256 }, true, [
    'encrypt',
    'decrypt'
  ]);

  const keyBuffer = await crypto.subtle.exportKey('raw', key);

  it('should work', async () => {
    const text = Buffer.from('hello world', 'utf-8');
    const result = await encrypt_aes_gcm(Buffer.from(keyBuffer), text);
    const value = new Uint8Array(result);

    const r1 = await decrypt_aes_gcm(Buffer.from(keyBuffer), result);
    const r2 = Buffer.from(
      await crypto.subtle.decrypt({ name: 'AES-GCM', iv: value.slice(0, 12) }, key, value.slice(12))
    );

    expect(r1).toStrictEqual(r2);
  });
});
