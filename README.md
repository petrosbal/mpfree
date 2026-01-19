![MpFree Logo](assets/mpfree_logo.png)

**MpFree** is a minimalist YouTube-to-MP3 downloader built with **Rust** and the **egui** framework. 

---

## Features
* **Native Performance:** Built for speed, memory safety and minimal overhead compared to interpreted alternatives.
* **Minimalist Interface:** Clean, distraction-free GUI, without ads or bloat.
* **Thread-Safe Downloads:** Uses multi-threading to ensure UI responsiveness.
* **Real-Time Feedback:** Uses MPSC channels for cross-thread status updates.

### Technical Implementation
* **Async Architecture:** Asynchronous background processing ensures long-running downloads don't freeze the main thread.
* **Zero-Dependency Design:** `yt-dlp` and `FFmpeg` are bundled directly within the executable, for a plug-and-play user experience.
* **Rust Ecosystem:** Developed using modern crates for robust error handling and efficient message passing.

---

## Project Roadmap

### *Coming soon...*
* **Directory Picker:** Custom output folder selection using native dialogs.
* **Real-time Progress Bar:** Visual percentage tracking for active downloads.
* **FFmpeg Independency:** Native Rust audio conversion to eliminate FFmpeg dependency.
* **Audio Quality Selector:** Toggle between 128kbps, 192kbps, and 320kbps.
* **Metadata & ID3 Tags:** Automatic embedding of Title, Artist, and Album Art.
* **Playlist Support:** Sequential processing of entire YouTube playlists.
* **Input Validation:** URL cleaning and error handling.
* **Auto-Updater:** Automated update checks for the application and downloader engine.
* **GitHub Actions:** Automated CI/CD pipelines for Windows and Linux releases.

---

## Installation Guide

MpFree is a portable, single-binary application. This means you don't need to install any external dependencies like Python, `yt-dlp`, or `FFmpeg`. Everything is packed inside the executable.

### Windows
1. Download `mpfree.exe` from the **Releases** page.
2. Move it to a folder of your choice (e.g., Desktop or Downloads).
3. Double-click to run.

> **Note:** Since the app is unsigned, Windows Defender will show a "Windows protected your PC" warning. Click **More info** and then **Run anyway**.

---

### Linux

1. Download `mpfree` from the **Releases** page.
2. Open your terminal and navigate to the download folder.
3. Give it execution permissions:

```bash
chmod +x mpfree
```

4. Run it:

```bash
./mpfree
```
