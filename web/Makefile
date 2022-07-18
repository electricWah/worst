
public/worst.js: ../src/ ../Cargo.toml ../lib/
	wasm-pack build --target web -d $(CURDIR)/public --no-default-features --features wasm

package_lock.json: package.json
	npm install

public/deps.js: wasmtest.js
	esbuild wasmtest.js --bundle --format=esm --outfile=$@

public/wasmtest.html: wasmtest.html
	cp $< $@

public/xterm.css: package_lock.json
	cp node_modules/xterm/css/xterm.css $@

all: package_lock.json public/worst.js public/deps.js public/wasmtest.html public/xterm.css

