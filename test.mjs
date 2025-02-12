import { encrypt_aes_gcm, decrypt_aes_gcm } from './index.js';

const key = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 256 },
  true,
  ['encrypt', 'decrypt']
);
const keyBuffer = await crypto.subtle.exportKey('raw', key);

console.log('key', Buffer.from(keyBuffer));
console.log('plain', Buffer.from('abc', 'utf-8'));

/**
 * @type {Buffer}
 */
const result = await encrypt_aes_gcm(
  Buffer.from(keyBuffer),
  Buffer.from('abc', 'utf-8')
);
const value = new Uint8Array(result);
console.log(value);
console.log(
  await decrypt_aes_gcm(Buffer.from(keyBuffer), result),
  await crypto.subtle.decrypt(
    { name: 'AES-GCM', iv: value.slice(0, 12) },
    key,
    value.slice(12)
  )
);
