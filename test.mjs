import { encrypt_aes_gcm } from './index.js';

const key = await crypto.subtle.generateKey(
  { name: 'AES-GCM', length: 256 },
  true,
  ['encrypt', 'decrypt']
);
const keyBuffer = await crypto.subtle.exportKey('raw', key);
console.log(Buffer.from(keyBuffer));
console.log(
  await encrypt_aes_gcm(Buffer.from('abc', 'utf-8'), Buffer.from(keyBuffer))
);
console.log(
  crypto.subtle.encrypt({ name: 'AES-GCM' }, key, Buffer.from('abc', 'utf-8'))
);
