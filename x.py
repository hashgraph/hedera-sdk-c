#!/usr/bin/env python3
import os, re, argparse, sys
from subprocess import Popen, check_call, PIPE, check_output, CalledProcessError
from shutil import copy2, copytree, rmtree
from sys import platform as _platform

def sh(command, silent=False, cwd=None, shell=True, env=None):
    if env is not None:
        env = dict(**os.environ, **env)

    if silent:
        p = Popen(
            command, shell=shell, stdout=PIPE, stderr=PIPE, cwd=cwd, env=env)
        stdout, _ = p.communicate()

        return stdout

    else:
        check_call(command, shell=shell, cwd=cwd, env=env)


def get_default_target():
    targets = sh("rustup target list", silent=True).decode()
    m = re.search(r"(.*?)\s*\(default\)", targets)
    t = m[1]

    if t == "x86_64-unknown-linux-gnu":
        t = "x86_64-unknown-linux-musl"

    return t


def build(release=False):
    additional_flags = ""
    default_target = get_default_target()
    targets = [
        "x86_64-apple-darwin",
        "x86_64-pc-windows-gnu",
        "x86_64-unknown-linux-musl"
    ]

    prefix = {
        "x86_64-apple-darwin": "",
        "x86_64-pc-windows-gnu": "x86_64-w64-mingw32-",
        "x86_64-unknown-linux-musl": "x86_64-linux-musl-"
    }

    artifact_static = {
        "x86_64-apple-darwin": "libhedera.a",
        "x86_64-pc-windows-gnu": "hedera.lib",
        "x86_64-unknown-linux-musl": "libhedera.a"
    }

    artifact_dynamic = {
        "x86_64-apple-darwin": "libhedera.dylib",
        "x86_64-pc-windows-gnu": "hedera.dll",
        "x86_64-unknown-linux-musl": "n/a"
    }

    flags = [
        "--verbose"
    ]

    if release:
        for target in targets:
            print(f" :: build hedera-sdk-c for {target}")

            if target != default_target:
                sh(["rustup", "target", "add", target],
                   shell=False,
                   silent=True)

            profile = "--release" if release else ''
            sh(f"cargo build --target {target} {profile}",
               env={
                   "CC": f"{prefix[target]}gcc",
                   "CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER": f"{prefix[target]}gcc",
                   "CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER": f"{prefix[target]}gcc",
               })

            if _platform == "linux" or _platform == "linux2":
                # linux
                sh(f"strip --strip-unneeded -d -x {artifact_static[target]}", cwd=f"target/{target}")
                if artifact_dynamic[target] != "n/a":
                    sh(f"strip --strip-unneeded -d -x {artifact_dynamic[target]}", cwd=f"target/{target}")
            elif _platform == "darwin":
                # MAC OS X
                sh(f"strip -Sx {artifact_static[target]}", cwd=f"target/{target}", silent=True)
                if artifact_dynamic[target] != "n/a":
                    sh(f"strip -Sx {artifact_dynamic[target]}", cwd=f"target/{target}", silent=True)
            elif _platform == "win32":
                # Windows
                sh(f"strip --strip-unneeded -d -x {artifact_static[target]}", cwd=f"target/{target}")
                if artifact_dynamic[target] != "n/a":
                    sh(f"strip --strip-unneeded -d -x {artifact_dynamic[target]}", cwd=f"target/{target}")
            elif _platform == "win64":
                # Windows 64-bit
                sh(f"strip --strip-unneeded -d -x {artifact_static[target]}", cwd=f"target/{target}")
                if artifact_dynamic[target] != "n/a":
                    sh(f"strip --strip-unneeded -d -x {artifact_dynamic[target]}", cwd=f"target/{target}")

            if not os.path.exists("libs/"):
                os.mkdir("libs/")
            if not os.path.exists(f"libs/{target}/"):
                os.mkdir(f"libs/{target}/")

            copy2(f"target/{target}/release/{artifact_static[target]}", f"libs/{target}/")
            if artifact_dynamic[target] != "n/a":
                copy2(f"target/{target}/release/{artifact_dynamic[target]}", f"libs/{target}/")
    else:
        target = default_target

        # For development; build only the _default_ target
        print(f" :: build hedera-sdk-c for {target}")
        sh(f"cargo build {additional_flags} --target {target}", cwd=".")

        if not os.path.exists("libs/"):
            os.mkdir("libs/")
        if not os.path.exists(f"libs/{target}/"):
            os.mkdir(f"libs/{target}/")

        # Copy _default_ lib over
        copy2(f"target/{target}/debug/{artifact_static[target]}", f"libs/{target}/")
        if artifact_dynamic[target] != "n/a":
            copy2(f"target/{target}/debug/{artifact_dynamic[target]}", f"libs/{target}/")


def commit():
    sha = sh("git rev-parse --short HEAD", cwd="vendor/hedera-sdk-c", silent=True).decode().strip()
    sh("git add ./vendor/hedera-sdk-c ./libs ./include")

    try:
        sh(f"git commit -m \"build libs/ and sync include/ from hedera-sdk-c#{sha}\"")
        sh("git push")

    except CalledProcessError:
        # Commit likely failed because there was nothing to commit
        pass


parser = argparse.ArgumentParser()
parser.add_argument(
    "-R", "--release", help="build in release mode", action="store_true")
parser.add_argument(
    "-C", "--commit", help="commit include/ and libs/", action="store_true")

argv = parser.parse_args(sys.argv[1:])

build(release=argv.release)

if argv.commit and argv.release:
    commit()