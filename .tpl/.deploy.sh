#!/usr/bin/env bash
mkdir ./$1
cp -r ./.tpl/* ./$1

cat <<- EOF > ./$1/Cargo.toml
[package]
name = "$1"
version = "0.0.1"
authors = ["Peter Frank <mdm23@gmx.de>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
gloo = "0.2.1"
js-sys = "0.3.37"
log = "0.4"
wasm-bindgen = "0.2.60"
web_logger = "0.2"

[dependencies.web-sys]
version = "0.3.37"
features = [
    "Document",
    "Window",
]

[dependencies.yew]
version = "0.16.2"
features = ["web_sys"]
EOF

cat <<- EOF > ./Cargo.toml
[workspace]
members = [
$(find . -maxdepth 1 -type d ! -name '.*' ! -name 'target' -printf '    "%f",\n' | sort)
]
EOF

cat <<- EOF > ./$1/index.html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=1" name="viewport" />
    <title>$1</title>
</head>
<body>
    <script type="module">
        import init from "./pkg/$1.js";
        (async () => await init())();
    </script>
</body>
</html>
EOF
