# imgup

> Upload images via APIs

[![Rust: Crates.io](https://img.shields.io/badge/dynamic/json?url=https://crates.io/api/v1/crates/imgup&query=$.crate.max_stable_version&prefix=v&label=crates.io&logo=rust&logoColor=white&color=orange)](https://crates.io/crates/imgup)
[![PyPI: Version](https://img.shields.io/pypi/v/images-upload-cli?logo=pypi&logoColor=white)](https://pypi.org/project/images-upload-cli)
[![AUR: version](https://img.shields.io/aur/version/imgup-bin?logo=archlinux&logoColor=white)](https://aur.archlinux.org/packages/imgup-bin)
[![GitHub: Release](https://img.shields.io/github/v/release/deadnews/imgup?logo=github&logoColor=white)](https://github.com/deadnews/imgup/releases/latest)
[![CI: Main](https://img.shields.io/github/actions/workflow/status/deadnews/imgup/main.yml?branch=main&logo=github&logoColor=white&label=main)](https://github.com/deadnews/imgup)
[![CI: Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/deadnews/imgup/refs/heads/badges/coverage.json)](https://github.com/deadnews/imgup)

**[Installation](#installation)** • **[Hostings](#hostings)** • **[Usage](#usage)** • **[Env Variables](#env-variables)**

## Installation

```sh
# PyPI
uv tool install images-upload-cli

# AUR
yay -S imgup-bin
```

## Hostings

| host                                  | key required | return example                                       |
| :------------------------------------ | :----------: | :--------------------------------------------------- |
| [beeimg](https://beeimg.com/)         |      -       | `https://beeimg.com/images/{id}.png`                 |
| [catbox](https://catbox.moe/)         |      -       | `https://files.catbox.moe/{id}`                      |
| [fastpic](https://fastpic.org/)       |      -       | `https://i120.fastpic.org/big/2022/0730/d9/{id}.png` |
| [freeimage](https://freeimage.host/)  |      +       | `https://iili.io/{id}.png`                           |
| [gyazo](https://gyazo.com/)           |      +       | `https://i.gyazo.com/{id}.png`                       |
| [imageban](https://imageban.ru/)      |      +       | `https://i2.imageban.ru/out/2022/07/30/{id}.png`     |
| [imgbb](https://imgbb.com/)           |      +       | `https://i.ibb.co/{id}/image.png`                    |
| [imgchest](https://imgchest.com/)     |      +       | `https://cdn.imgchest.com/files/{id}.png`            |
| [imgur](https://imgur.com/)           |      -       | `https://i.imgur.com/{id}.png`                       |
| [lensdump](https://lensdump.com/)     |      +       | `https://i.lensdump.com/i/{id}.png`                  |
| [pixeldrain](https://pixeldrain.com/) |      +       | `https://pixeldrain.com/api/file/{id}`               |
| [pixhost](https://pixhost.to/)        |      -       | `https://img75.pixhost.to/images/69/{id}_img.png`    |
| [ptpimg](https://ptpimg.me/)          |      +       | `https://ptpimg.me/{id}.png`                         |
| [sxcu](https://sxcu.net/)             |      -       | `https://sxcu.net/{id}.png`                          |
| [thumbsnap](https://thumbsnap.com/)   |      +       | `https://thumbsnap.com/i/{id}.png`                   |
| [tixte](https://tixte.com/)           |      +       | `https://{domain}.tixte.co/r/{id}.png`               |
| [uplio](https://upl.io/)              |      +       | `https://upl.io/i/{id}.png`                          |
| [uploadcare](https://uploadcare.com/) |      +       | `https://ucarecdn.com/{id}/img.png`                  |
| [vgy](https://vgy.me/)                |      +       | `https://i.vgy.me/{id}.png`                          |
| [zpic](https://zpic.biz/)             |      +       | `https://zpi.cx/b/{id}.png`                          |

## Usage

```sh
Upload images via APIs

Usage: imgup [OPTIONS] <IMAGES>...

Arguments:
  <IMAGES>...  Image files to upload

Options:
  -H, --hosting <HOSTING>          Hosting service to use [default: imgur]
                                   [possible values: beeimg, catbox, fastpic, freeimage,
                                   gyazo, imageban, imgbb, imgchest, imgur, lensdump,
                                   pixeldrain, pixhost, ptpimg, sxcu, thumbsnap, tixte,
                                   uplio, uploadcare, vgy, zpic]
  -f, --format <FORMAT>            Output format for the links [default: plain]
                                   [possible values: plain, bbcode, html, markdown]
  -t, --thumbnail                  Create captioned thumbnails
  -n, --notify                     Send desktop notification on completion
      --no-clipboard               Disable copying the result to the clipboard
      --env-file <ENV_FILE>        Path to .env file. Overrides default config path
  -j, --jobs <JOBS>                Max concurrent uploads [default: 4]
  -v, --verbose...                 Increase verbosity (-v for info, -vv for debug)
  -h, --help                       Print help
  -V, --version                    Print version
```

## Env Variables

```ini
FREEIMAGE_KEY=
GYAZO_TOKEN=
IMAGEBAN_TOKEN=
IMGBB_KEY=
IMGCHEST_KEY=
IMGUR_CLIENT_ID=
LENSDUMP_KEY=
PIXELDRAIN_KEY=
PTPIMG_KEY=
THUMBSNAP_KEY=
TIXTE_KEY=
UPLIO_KEY=
UPLOADCARE_KEY=
VGY_KEY=
ZPIC_KEY=
```

You can set these in environment variables, or in `.env` file:

- Unix: `~/.config/imgup/.env`
- MacOS: `~/Library/Application Support/imgup/.env`
- Windows: `C:\Users\<user>\AppData\Roaming\imgup\.env`
