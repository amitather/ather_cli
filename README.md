# Ather_CLI

ATHER_CLI is a sample mqtt test library.

## RUST Installation

First install `rustup`
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Set correct `Rust version`
```bash
rustup override set 1.44.0
```

This makes the `Rust 1.44.0` applicable for current project/
## Environment variables

```bash
export VI_BROKER_CA_PATH=<path_to_broker_ca>
export VI_CLIENT_CERT_PATH = <path to x509 cert of client>
export VI_CLIENT_PRIVATE_KEY_PATH = <path to unencrypted private key of client>
```

## ECC KEY FORMAT

The ECC key must be unencrypted and in `pkcs8 format` and PEM encoding.

 "-----BEGIN PRIVATE KEY-----",

 "-----END PRIVATE KEY-----",

PKCS8 encoding keys have the above format.

```bash
openssl pkcs8 -topk8 -nocrypt -in tradfile.pem -out p8file.pem
```
The above command converts an `PEM ECC` key to `PEM PKCS8 format`. Note the final keys must be un-encrypted.

## Running
 ```bash
cargo run
```

## License
[MIT](https://choosealicense.com/licenses/mit/)