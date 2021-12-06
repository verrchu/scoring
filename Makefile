BATS_CASES := ./test/cases
BATS_CASE_INPUT_HEADER := "type,client,tx,amount"
BATS_CASE_OUTPUT_HEADER := "client,available,held,total,locked"

docs:
	@ cargo doc --open

submodules:
	@ git submodule update --init --recursive >/dev/null

release:
	@ cargo build --release

release_tools:
	@ cargo build --features tools --release --bins

bats: release submodules
	@ ./test/bats/bin/bats test/test.bats

new_bats_case:
ifndef NAME
	$(error NAME is not set)
endif
	@ mkdir ${BATS_CASES}/${NAME}/
	@ echo ${BATS_CASE_INPUT_HEADER} > ${BATS_CASES}/${NAME}/input.csv
	@ echo ${BATS_CASE_OUTPUT_HEADER} > ${BATS_CASES}/${NAME}/output.csv

bench: release_tools
	@ $(eval EVENT_LOG := $(shell mktemp))
	@ printf "Event log will be generated in ${EVENT_LOG}\n"

	@ printf "\n10 account; 1000 events\n"
	@ printf " - Gnenerating test data\n"
	@ ACCOUNTS=10 EVENTS=1000 EVENT_LOG=${EVENT_LOG} .scripts/generate_event_log.sh
	@ printf " - Running test\n\n"
	@ EVENT_LOG=${EVENT_LOG} time .scripts/run_capture_out.sh

	@ printf "\n100 account; 10000 events\n"
	@ printf " - Gnenerating test data\n"
	@ ACCOUNTS=100 EVENTS=10000 EVENT_LOG=${EVENT_LOG} .scripts/generate_event_log.sh
	@ printf " - Running test\n\n"
	@ EVENT_LOG=${EVENT_LOG} time .scripts/run_capture_out.sh

	@ printf "\n1000 account; 100000 events\n"
	@ printf " - Gnenerating test data\n"
	@ ACCOUNTS=1000 EVENTS=100000 EVENT_LOG=${EVENT_LOG} .scripts/generate_event_log.sh
	@ printf " - Running test\n\n"
	@ EVENT_LOG=${EVENT_LOG} time .scripts/run_capture_out.sh

	@ printf "\n10000 account; 1000000 events\n"
	@ printf " - Gnenerating test data\n"
	@ ACCOUNTS=10000 EVENTS=1000000 EVENT_LOG=${EVENT_LOG} .scripts/generate_event_log.sh
	@ printf " - Running test\n\n"
	@ EVENT_LOG=${EVENT_LOG} time .scripts/run_capture_out.sh
