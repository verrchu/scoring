submodules:
	@ git submodule update --init --recursive

bats: submodules
	@ ./test/bats/bin/bats test/test.bats
