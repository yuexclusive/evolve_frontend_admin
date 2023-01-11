target=aarch64-unknown-linux-musl
ip=`ifconfig en0 | grep inet | cut -d " " -f 2`

.PHONY: clean
clean:
	docker images -f "dangling=true" | grep -v kindest | awk 'NR!=1{print $$3}' | xargs docker rmi

.PHONY: css-build
css-build:
	cd mybulma && npm run css-build

.PHONY: css-watch
css-watch:
	cd mybulma && npm run css-watch


name=`find ./dist -name "*.wasm" | sed 's/.\/dist\///g'`
.PHONY: build
build:
	trunk build --release
	wasm-opt -Oz -o ./dist/${name}_copy ./dist/${name}
	mv ./dist/${name}_copy ./dist/${name}
	
.PHONY: image
image: build
	docker build --no-cache -f admin.dockerfile -t yuexclusive/evolve_frontend_admin:latest .
	make clean

.PHONY: run
run: image
	docker run --rm -it -p 8882:80 yuexclusive/evolve_frontend_admin:latest