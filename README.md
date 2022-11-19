# Build for Linux on Windows
## Prerequisites
- [Docker](https://www.docker.com/) for creating our build-environment.

## Setup
Build an image for our build-environment:
`docker build buildenv -t ragecoop-buildenv`

## Build
Enter build environment:
- Windows (PowerShell): `docker run --rm -it -v "${pwd}:/root/env" ragecoop-buildenv`

### Commands
Build for release:
`cargo build --release`

To leave the build environment, enter `exit`.

## Cleanup
Remove the build-environment image:
`docker rmi ragecoop-buildenv -f`