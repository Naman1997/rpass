## rpass

### This project is still a work in progress!

An extremely simple password manager tui written in rust.

rpass uses gnugpg under the hood to encrypt your credentials with PGP encryption and pushes your encrypted credentials to a git repo managed by you.

Note: Please make sure to backup your keys. rpass does not manage your keys - you should use gpg for this.

Note: It's a good idea to make sure that the git repo you provide to rpass is private.