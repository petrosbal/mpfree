# Changelog

All notable changes to this project will be documented in this file.
The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).


## [Unreleased]
* **Real-time Progress Bar:** Visual percentage tracking for active downloads.
* **Direct Youtube Search and Discovery:** Search query support to retrieve and list top YouTube results, eliminating URL dependence.
* **FFmpeg Independency:** Native Rust audio conversion to eliminate FFmpeg dependency.
* **Audio Quality Selector:** Toggle between 128kbps, 192kbps, and 320kbps.
* **Metadata & ID3 Tags:** Automatic embedding of Title, Artist, and Album Art.
* **Playlist Support:** Sequential processing of entire YouTube playlists.
* **Input Validation:** URL cleaning and error handling.
* **Auto-Updater:** Automated update checks for the application and downloader engine.

## [0.1.1] - 2026-01-19

### Added
- GNU General Public License v3.0.
- Custom output folder selection using native dialogs.
- Source code split into multiple files for modularity.
- mpFree logo inside the application.
- Icon for the application window and executable.
- Makefile Windows target for easier building on Windows systems.

## [0.1.0] - 2026-01-19
- Initial release.
- Minimal GUI with input field and download button.
- Threads for concurrent downloading and UI updates.
- Binary embedding for easy installation.
- Automated CI/CD pipelines for multi-platform support (Windows and Linux releases).