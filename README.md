# rustatusbar
*Configurable statusbar in Rust for Xorg server using xsetroot*

## Caveats
- Dependencies: `Rust`, `xsetroot`
- This library has been developed on and for Linux following open source philosophy.

## Installation
- First step:
```bash
git clone https://github.com/javiorfo/rustatusbar
cd rustatusbar
sudo make clean install
```

- In your **~/.xinitrc** to start in every login
```bash
rustatusbar &
```

## Overview
| Component | rustatusbar | NOTE |
| ------- | ------------- | ---- |
| CPU usage | :heavy_check_mark: | percentage |
| RAM usage | :heavy_check_mark: | percentage |
| TEMPERATURE | :heavy_check_mark: | Celcious |
| DISK USAGE | :heavy_check_mark: | percentage |
| VOLUME LEVEL | :heavy_check_mark: | Level and Mute status |
| BATTERY LEVEL | :heavy_check_mark: | percentage |
| WEATHER | :x: | |
| NETWORK STATUS | :heavy_check_mark: | Up or down |
| DATE | :heavy_check_mark: | could be custimizable |

## Customizable
- By default the statusbar contains: **cpu usage, memory usage, temperature, disk usage, volume, network status and datetime**
- For a custom configuration put this file [config.toml](https://github.com/javiorfo/rustatusbar/blob/master/examples/config.toml) in your `~/.config/rustatusbar/config.toml` and edit it to change values or delete a component.

## Screenshots

<img src="https://github.com/javiorfo/img/blob/master/xtatusbar/xtatusbar.png?raw=true" alt="rustatusbar" />

---

### Donate
- **Bitcoin** [(QR)](https://raw.githubusercontent.com/javiorfo/img/master/crypto/bitcoin.png)  `1GqdJ63RDPE4eJKujHi166FAyigvHu5R7v`
- [Paypal](https://www.paypal.com/donate/?hosted_button_id=FA7SGLSCT2H8G)
