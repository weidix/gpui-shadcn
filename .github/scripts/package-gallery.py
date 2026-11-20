"""Validate the native Gallery executable and create its distribution archive."""

import os
import plistlib
import shutil
import struct
import subprocess
import sys
import tempfile
from pathlib import Path


def verify_windows(binary):
    with binary.open("rb") as stream:
        dos = stream.read(64)
        if dos[:2] != b"MZ":
            raise ValueError("Expected an MZ executable")
        stream.seek(struct.unpack_from("<I", dos, 60)[0])
        pe = stream.read(24 + 70)
    if (
        pe[:4] != b"PE\0\0"
        or struct.unpack_from("<H", pe, 4)[0] != 0x8664
        or struct.unpack_from("<H", pe, 24)[0] != 0x20B
        or struct.unpack_from("<H", pe, 24 + 68)[0] != 2
    ):
        raise ValueError("Expected a Windows x64 GUI executable")


def install_macos_app(binary, package, platform):
    arch = "arm64" if platform == "macos-arm64" else "x86_64"
    subprocess.run(["lipo", binary, "-verify_arch", arch], check=True)
    contents = package / "GPUI Shadcn Gallery.app" / "Contents"
    executable = contents / "MacOS" / binary.name
    executable.parent.mkdir(parents=True)
    shutil.copy2(binary, executable)
    with (contents / "Info.plist").open("wb") as stream:
        plistlib.dump(
            {
                "CFBundleExecutable": binary.name,
                "CFBundleIdentifier": "io.github.weidix.gpui-shadcn",
                "CFBundleName": "GPUI Shadcn Gallery",
                "CFBundlePackageType": "APPL",
                "NSHighResolutionCapable": True,
            },
            stream,
        )
    subprocess.run(["codesign", "--force", "--sign", "-", contents.parent], check=True)
    subprocess.run(["codesign", "--verify", "--strict", contents.parent], check=True)


def verify_linux(binary):
    with binary.open("rb") as stream:
        header = stream.read(20)
    if header[:6] != b"\x7fELF\x02\x01" or struct.unpack_from("<H", header, 18)[0] != 62:
        raise ValueError("Expected a Linux x64 ELF executable")
    libraries = subprocess.check_output(["ldd", binary], text=True)
    print(libraries, end="")
    if "not found" in libraries:
        raise ValueError("The Gallery executable has unresolved shared libraries")


def package_gallery(platform, target):
    binary_name = "gpui-shadcn.exe" if platform == "windows-x64" else "gpui-shadcn"
    target_root = Path(os.environ.get("CARGO_TARGET_DIR", "target"))
    binary = target_root / target / "release" / binary_name
    dist = Path("dist").resolve()
    dist.mkdir(exist_ok=True)
    name = f"gpui-shadcn-{platform}"

    with tempfile.TemporaryDirectory() as directory:
        package = Path(directory) / name
        package.mkdir()
        shutil.copy2("LICENSE", package)
        assets = Path("crates/gpui-shadcn/assets")
        shutil.copy2(assets / "OFL.txt", package / "Geist-OFL.txt")
        shutil.copy2(assets / "LUCIDE-LICENSE.txt", package)
        shutil.copy2("THIRD-PARTY-LICENSES.html", package)
        if commit := os.environ.get("GITHUB_SHA"):
            (package / "source-commit.txt").write_text(f"{commit}\n", encoding="utf-8")

        if platform in ("macos-arm64", "macos-x64"):
            install_macos_app(binary, package, platform)
            subprocess.run(
                [
                    "ditto", "-c", "-k", "--norsrc", "--keepParent",
                    package, dist / f"{name}.zip",
                ],
                check=True,
            )
            return

        if platform == "windows-x64":
            verify_windows(binary)
            archive_format = "zip"
        elif platform == "linux-x64":
            verify_linux(binary)
            archive_format = "gztar"
        else:
            raise ValueError(f"Unsupported platform: {platform}")

        shutil.copy2(binary, package)
        shutil.make_archive(str(dist / name), archive_format, directory, name)


if __name__ == "__main__":
    package_gallery(*sys.argv[1:])
