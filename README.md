# GXAndria

Game library (Alexdrandria, ba-dum-tsss) helper for Heroic Game Launcher.

## Motivation
Heroic Game Launcher lacks the ability to auto-categories games based on their genres (and other metadata).
It's such an essential feature that is somehow surprising that's not implemented yet.

This is simple script that will create custom categories, based on genres, in your Heroic Game Launcher using
IGDB's API as metadata source.

## Requirements

You'll need IGDB free API access, which requires simply a Twitch account.
See [IGDB API documentation](https://api-docs.igdb.com/) for details on how to get your client ID and secret.

After that, simply run the program with the two following environment variables:

```bash
export IGDB_CLIENT_ID=your_client_id
export IGDB_CLIENT_SECRET=your_client_secret
```
optionally, you can specify the path for GXAndria internal SQLITE database (for caching purposes) via env var:
```bash
export SQLITE_PATH=/path/to/your/gxandria.db
```
default is `~/.local/share/gxandria/store.db`.

## Install

Prebuilt linux binaries will be available in Github releases.

A Nix package is in the works, stay tuned! :)
