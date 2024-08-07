# pbskids-dl rust
Documentation coming soon

## Build Dependencies

Rust (version > 1.63), CMake (version > 3.15), Git and a C++17 compiler need to be installed and in your PATH for a cross-platform build from source. [Ninja](https://github.com/ninja-build/ninja) is recommended, but not required. This crate also offers a bundled form of fltk on selected x86_64 and aarch64 platforms (Windows (msvc and gnu), MacOS, Linux), this can be enabled using the fltk-bundled feature flag as mentioned in the usage section (this requires curl and tar to download and unpack the bundled libraries).

- X11 and OpenGL development headers need to be installed for development. The libraries themselves are normally available on Linux/BSD distros that support a graphical user interface.

For Debian-based distributions, you can install the required libraries with this command:
```bash
sudo apt-get install libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev
```
On RHEL-based distributions:
```bash
sudo yum groupinstall "X Software Development" && sudo yum install pango-devel libXinerama-devel libstdc++-static
```
On Arch-based distributions:
```bash
sudo pacman -S libx11 libxext libxft libxinerama libxcursor libxrender libxfixes pango cairo libgl mesa --needed
```
On Alpine Linux:
```bash
apk add pango-dev fontconfig-dev libxinerama-dev libxfixes-dev libxcursor-dev mesa-gl
```
On Nix**OS**, this `nix-shell` environment can be used:
```bash
nix-shell --packages rustc cmake git gcc xorg.libXext xorg.libXft xorg.libXinerama xorg.libXcursor xorg.libXrender xorg.libXfixes libcerf pango cairo libGL mesa pkg-config
```

Cross-compiling is *not* possible due to openssl-sys requiring the C headers of openssl for the target.

## Runtime Dependencies
- You need X11 libraries, as well as pango and cairo for drawing (and OpenGL if you want to enable the enable-glwindow feature):
```bash
apt-get install -qq --no-install-recommends libx11-6 libxinerama1 libxft2 libxext6 libxcursor1 libxrender1 libxfixes3 libcairo2 libpango-1.0-0 libpangocairo-1.0-0 libpangoxft-1.0-0 libglib2.0-0 libfontconfig1 libglu1-mesa libgl1
```
Note that if you installed the build dependencies, it will also install the runtime dependencies automatically as well.

Also note that **both** X11 and OpenGL should have these libraries built-in. This list can be useful if you want to test your already built package in CI/Docker (where there is no graphical user interface).