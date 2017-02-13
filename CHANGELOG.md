# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) and this
project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

 - Instead of prompting with an error description when the DNS fails to bind,
 it will now attempt to bind the DNS server port in a loop. This behavior is
 more desirable for Dolphin users, since they will not need to reopen the
 application just before the game times out.

## 1.1.1

### Added

 - On next release, AppVeyor will deploy an i686 Windows build to GitHub
 Releases.
 - When a socket fails to bind, a better error description will be shown.

## 1.1.0

This is a new minor release that breaks on previous configuration files, please
adjust your configuration appropriately.

### Added

 - Redirection now takes a configurable value in telepipe.toml called
 `proxy_server_address` -- this is used by both the DNS server and the proxy
 itself. This addresses a problem with the proxy only redirecting to
 192.168.150.1 (sorry! oversight. #2)

## 1.0.0

This is the initial release. There is no specialized message filtering beyond
encryption and redirects.
