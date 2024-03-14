## Steps:

=> rpass init
- Ask user for a bare git repo url and inform that the default git login will be used to pull and push to their gpg repo
- Attempt to clone the repo in config dir. In case of failure, exit with err
- Method for init that generates a new default key pair
- Run the init method if list-keys is empty (probably 1st run)
- If list-keys is not empty, then ask if the user will like to use an existing key as default (some sort of tui selection is needed here)
- save to disk the default keyname in config dir
- save to disk that init has completed successfully
- ask user to export the public and private gpg key to some backup

=> rpass save
- CLI flag for key to use by name (overrides the usage of default key) (probably --set-default-key)
- Ask user for service_name, username and password. API keys are not supported as of now
- In case the service_name and username combination already exists, then ask confirmation to override
- If override is approved, delete the old encrypted file
- Sync the encrypted passwords in the following dir structure and do a git push

```
key_name
└── service_name
    └── username.gpg
```

- Format of the decrypted file will look like:

```
<username>
<password>
```

=> rpass fetch
- CLI flag for key to use by name (overrides the usage of default key) (probably --set-default-key)
- Print the key name that will be used to fetch
- Provide a tui for service selection
- Again provide tui if more than one file exists for that service
- If decryption is successful, then first copy the username to clipboard and wait for enter key to be pressed
- After the enter key is pressed, copy the password into the clipboard as well