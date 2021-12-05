setup() {
    load 'test_helper/bats-support/load'
    load 'test_helper/bats-assert/load'

    CASES=./test/cases
}

# TODO: output CSV header
scoring() {
    cargo run --release -- $CASES/$1/input.csv 2>/dev/null | tail -n +2 | sort
}

# TODO: output CSV header
result() {
    tail -n +2 $CASES/$1/output.csv | sort
}

@test "example" {
    run scoring example
    assert_output "$(result example)"
}
