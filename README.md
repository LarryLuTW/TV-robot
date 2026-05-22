# TV-robot

This project is used to control computer's up, down, left, and right, and volume buttons on my phone. If you want to know the development process, you can read the article [不想離開沙發，只好自己寫一個電腦遙控器了](https://medium.com/starbugs/how-to-make-a-computer-controller-7f8ffcdbe993).

## [Demo on Youtube](https://www.youtube.com/watch?v=aIx-Li1m-3c)

[![Youtube Demo Video](https://user-images.githubusercontent.com/10403741/113395919-39aab680-93cd-11eb-8a72-36f374df1927.png)](https://www.youtube.com/watch?v=aIx-Li1m-3c)

## Install

### Download prebuilt binary (recommended)

Grab the latest binary from the [Releases page](https://github.com/LarryLuTW/TV-robot/releases/latest):

- **macOS (Apple Silicon)**: `tv-robot-macos-aarch64`
- **Windows (x86_64)**: `tv-robot-windows-x86_64.exe`

#### macOS

```sh
chmod +x tv-robot-macos-aarch64
./tv-robot-macos-aarch64
```

On first run, macOS may block the binary because it's unsigned. Go to *System Settings → Privacy & Security* and click "Open Anyway".

#### Windows

Double-click the `.exe`, or run it from PowerShell:

```powershell
.\tv-robot-windows-x86_64.exe
```

On first run:

1. **SmartScreen warning**: Windows will warn that the publisher is unknown. Click *More info* → *Run anyway*.
2. **Firewall prompt**: Windows Defender Firewall will ask to allow inbound connections on port 3000. Click *Allow* (at least for private networks).
3. If the QR code in the terminal looks garbled, switch to UTF-8 with `chcp 65001` (Windows Terminal usually handles it without this).

> **Note:** The sleep button is disabled on Windows — it's only supported on macOS.

### Build from source

```sh
cargo install --git https://github.com/LarryLuTW/TV-robot
```

Requires the Rust toolchain. On Linux you'll also need `libxdo-dev`.

## Run

#### 1. Connect your phone to the same wifi as your computer 

#### 2. Run `tv-robot` command and the QRCode will appear on the terminal.

<img width="697" alt="Screen Shot 2021-03-31 at 3 49 53 PM" src="https://user-images.githubusercontent.com/10403741/113109894-046d5f80-9239-11eb-9f09-61dc372218c0.png">

#### 3. Scan this QRCode with your phone to control the computer.

<img width="328" alt="Screen Shot 2021-03-31 at 3 58 18 PM" src="https://user-images.githubusercontent.com/10403741/113110769-08e64800-923a-11eb-81cb-7b0abb651cde.png">

## Upgrade Dependencies

To upgrade all dependencies to their latest versions, run:

```sh
cargo upgrade -i && cargo update
```
