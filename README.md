
# Kirino Media Server

A lightweight, modular alternative to [Plex](https://plex.tv) and [Jellyfin](https://jellyfin.org)

---

## Installation

For the time being, Kirino is a little difficult to install. We'd love your help packaging!

Before we get started, you'll need the following non-Rust dependencies:
 - libffmpegthumbnailer

### Linux

For Gentoo users:

 > ```shell
 > # Add our overlay
 > eselect repository add selfhosted-overlay git https://github.com/kirino-org/selfhosted-overlay
 > # Merge KMS
 > emerge --ask media-video/kirino
 > ```

Other distros (with Cargo):
 > ```shell
 > cargo install kirino
 > ```

### Windows

---

[Forgejo mirror](https://kirino.io/kirino/kirino)

 > **Still in early development**
 >
 > Not at all usable

---

