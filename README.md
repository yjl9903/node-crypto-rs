# node-crypto-rs

[![CI](https://github.com/yjl9903/node-crypto-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/yjl9903/node-crypto-rs/actions/workflows/ci.yml)

## Installation

```bash
npm i node-crypto-rs
```

## Usage

```ts
import { encrypt_aes_gcm, decrypt_aes_gcm } from 'node-crypto-rs'

const key = await crypto.subtle.exportKey(
  'raw',
  await crypto.subtle.generateKey({ name: 'AES-GCM', length: 256 }, true, ['encrypt', 'decrypt'])
);

const encrypted = await encrypt_aes_gcm(Buffer.from(key), Buffer.from('hello', 'utf-8'))
const plainText = await decrypt_aes_gcm(Buffer.from(key), encrypted)
```

## License

MIT License © [2025] [XLor](https://github.com/yjl9903)
