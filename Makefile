BATS_CASES := ./test/cases
BATS_CASE_INPUT_HEADER := "type,client,tx,amount"
BATS_CASE_OUTPUT_HEADER := "client,available,held,total,locked"

submodules:
	@ git submodule update --init --recursive

release:
	@ cargo build --release &>/dev/null

bats: release submodules
	@ ./test/bats/bin/bats test/test.bats

new_bats_case:
ifndef NAME
	$(error NAME is not set)
endif
	@ mkdir ${BATS_CASES}/${NAME}/
	@ echo ${BATS_CASE_INPUT_HEADER} > ${BATS_CASES}/${NAME}/input.csv
	@ echo ${BATS_CASE_OUTPUT_HEADER} > ${BATS_CASES}/${NAME}/output.csv

bench:
	@ $(eval EVENT_LOG := $(shell mktemp -t event_log))
	@ echo "Event log will be generated in ${EVENT_LOG}"
	@ cargo run --bin generate_event_log -- --accounts 10 --events 1000 --file ${EVENT_LOG}
