setup() {
    load 'test_helper/bats-support/load'
    load 'test_helper/bats-assert/load'
}

scoring() {
    cargo run -- $1 2>/dev/null
}

@test "example" {
    run scoring ./test/data/example/input.csv
    assert_output "$(cat ./test/data/example/output.csv)"
}
