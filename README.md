# rust-jwt-test

Testing jwt in Rust using [frank_jwt](https://github.com/GildedHonour/frank_jwt).

## How to generate a key for testing it?
```bash
ssh-keygen -t rsa -b 4096 -m PEM -f jwtRS256.key
# Don't add passphrase
```
## Then just execute
```bash
openssl rsa -in jwtRS256.key -pubout -outform PEM -out jwtRS256.key.pub
```

## Check what is inside
```bash
cat jwtRS256.key
```
```bash
cat jwtRS256.key.pub
```
