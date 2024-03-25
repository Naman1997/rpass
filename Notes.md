# Create key
gpg --full-gen-key

# Encrypt secret
gpg --encrypt --recipient <key_name> my_secret

## my_secret is the file name

# Decrypt secret
gpg --decrypt my_secret.gpg

# Export public key
gpg --armor --export <key_name> > pubkey.asc

# Export private key
gpg --export-secret-keys <key_name> > privatekey.asc

# Import public/private keys
gpg --import <file_name>

# Creating binary from a bash script
`https://www.simplified.guide/bash/compile-script`