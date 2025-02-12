export declare function encrypt_aes_gcm(key: Buffer, plain: Buffer): Promise<Buffer>;

export declare function decrypt_aes_gcm(key: Buffer, encrypted: Buffer): Promise<Buffer>;
