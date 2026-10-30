# Playit.gg Partner Application — intmc

---

## 1. Application Information

| Field | Value |
|-------|-------|
| **App Name** | intmc |
| **Website** | https://github.com/SSIT2051/IntMc |
| **App Description** | See detailed description below |

---

## 2. Detailed App Description

### What is intmc?

**intmc** is a mobile-first Minecraft server hosting application designed specifically for **Android and ARM-based mobile devices**. It enables users to run fully functional Minecraft servers directly on their phones, tablets, or low-end ARM devices (such as Raspberry Pi, ARM Chromebooks, and other single-board computers) without requiring a traditional desktop or VPS.

### Core Features

- **Multiple Server Instances**: Create and manage multiple independent Minecraft servers simultaneously
- **Rust-Based Server Software Support**: Native support for modern, high-performance Rust-based Minecraft server implementations:
  - **PumpkinMC** — Async, modern Rust server with plugin support
  - **SteelMC** — Lightweight, fast Rust server
  - **Rosewater** — High-performance Rust server with focus on compatibility
  - **Glowstone-rs** — Rust port of the Glowstone server
- **Plugin Marketplace**: Built-in marketplace for discovering, installing, and managing server plugins
- **LAN Play**: Local network multiplayer support for offline or local-only sessions
- **Low-Resource Optimization**: Engineered for devices with limited RAM, CPU, and storage
- **ARM-Native**: Optimized for ARM64/ARMv7 architectures; no x86 emulation required

### Target Audience

- Mobile users who want to host Minecraft servers on-the-go
- Users with low-end or older devices (Android phones, tablets, ARM SBCs)
- Developers and hobbyists experimenting with Rust-based Minecraft server software
- Educational settings where students learn server administration on accessible hardware
- Communities in regions with limited access to traditional hosting infrastructure

---

## 3. Playit.gg Integration

### How intmc Uses Playit.gg

intmc integrates **Playit.gg as the primary tunneling solution** to expose locally hosted Minecraft servers to the public internet. This enables players worldwide to join servers running on mobile/ARM devices without port forwarding, static IPs, or VPS costs.

### Integration Workflow

| Event | Playit.gg Action |
|-------|------------------|
| **User creates a new server** | Automatically create a **TCP tunnel** via Playit.gg API/CLI agent |
| **Server starts** | Tunnel becomes active; assign unique **public IP:port** to the server |
| **Server stops** | Tunnel remains allocated (for quick restart) or enters idle state |
| **User deletes a server** | **Automatically delete** the associated Playit.gg tunnel (cleanup) |
| **Playit.gg unavailable** | **Fallback to LAN-only mode**; notify user; queue tunnel creation for retry |

### Technical Implementation Details

- **Tunnel Type**: TCP tunnels (Minecraft uses TCP on port 25565 by default)
- **Uniqueness**: Each server instance receives its **own dedicated public IP:port combination** — no port sharing or multiplexing
- **Automation**: Fully automated via Playit.gg **REST API** (preferred) or **CLI agent** as fallback
- **Authentication**: Secure API token stored in encrypted local storage; never hardcoded
- **Error Handling**:
  - Retry with exponential backoff on API failures
  - User notification on persistent failures
  - Graceful degradation to LAN mode
- **Resource Cleanup**: Guaranteed tunnel deletion on server deletion (via `Drop`/`defer` patterns in Rust backend)

### Why Playit.gg?

- **Zero-config tunneling**: No router access, no port forwarding, no firewall rules
- **ARM/ARM64 support**: Playit.gg agent runs natively on the same devices intmc targets
- **Reliability**: Established infrastructure with global POPs
- **Cost-effective**: Free tier sufficient for most intmc users; aligns with "free hosting on mobile" value prop
- **Developer-friendly API**: Straightforward tunnel lifecycle management

---

## 4. Owner Information

| Field | Value |
|-------|-------|
| **Owner Legal Name** | [User to fill] |
| **Owner Country** | [User to fill] |
| **Owner Profile URL** | https://github.com/SSIT2051 |

> **Note**: The owner is the maintainer of the intmc project at https://github.com/SSIT2051/IntMc

---

## 5. Company Details (If Applicable)

- **Company Name**: N/A (Individual developer project)
- **Company Website**: N/A
- **Company Registration Number**: N/A
- **Tax ID / VAT Number**: N/A

> intmc is currently an open-source project maintained by an individual. No legal entity is associated at this time.

---

## 6. Technical Integration Summary (For Playit.gg Review)

| Aspect | Details |
|--------|---------|
| **Integration Method** | Playit.gg REST API (primary) / CLI agent (fallback) |
| **Tunnel Protocol** | TCP |
| **Tunnel per Server** | 1:1 (unique public IP:port per server instance) |
| **Automation Level** | Fully automated (create on server create, delete on server delete) |
| **Fallback Strategy** | LAN-only play when Playit.gg unavailable; auto-retry tunnel creation |
| **Auth Management** | User-provided API token; encrypted local storage |
| **Rate Limit Handling** | Exponential backoff + user notification |
| **Cleanup Guarantee** | Tunnel deletion tied to server deletion lifecycle |

---

## 7. Compliance & Usage

- **Expected Concurrent Tunnels**: 1–5 per typical user; up to 20 for power users
- **Bandwidth per Tunnel**: Typical Minecraft traffic (~50–200 KB/s per player)
- **Geographic Distribution**: Global (users host from anywhere; players connect globally)
- **Abuse Prevention**:
  - Server owner must authenticate with Playit.gg account
  - Tunnel tied to specific server process lifecycle
  - No persistent tunnels without active server
  - Local-only fallback prevents orphaned tunnels

---

## 8. Links & References

- **Project Repository**: https://github.com/SSIT2051/IntMc
- **Playit.gg Website**: https://playit.gg
- **Playit.gg API Docs**: https://api.playit.gg (or current developer docs URL)

---

## 9. Declaration

I confirm that:
- intmc will use Playit.gg tunnels **only for legitimate Minecraft server hosting**
- Tunnels will be **created and deleted automatically** in direct response to user actions
- No tunnels will be left running without an active associated server
- Integration will respect Playit.gg **rate limits, terms of service, and fair use policies**
- Users will be informed that tunneling is provided by Playit.gg

---

**Prepared for Playit.gg Partner Program Review**  
**Date**: 2026-10-30  
**Contact**: https://github.com/SSIT2051