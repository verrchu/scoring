setup() {
    load 'test_helper/bats-support/load'
    load 'test_helper/bats-assert/load'

    CASES=./test/cases
}

# TODO: output CSV header
scoring() {
    output="$(cargo run --release -- $CASES/$1/input.csv 2>/dev/null)"; assert_equal $? 0
    echo "$output" | tail -n +2 | sort
}

# TODO: output CSV header
result() {
    tail -n +2 $CASES/$1/output.csv | sort
}

@test "example" {
    run scoring example
    expected="$(result example)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "basic successful operations single account" {
    run scoring basic_successful_operations_single_account
    expected="$(result basic_successful_operations_single_account)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "basic successful operations multiple accounts" {
    run scoring basic_successful_operations_multiple_accounts
    expected="$(result basic_successful_operations_multiple_accounts)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "insufficient funds withdrawal" {
    run scoring insufficient_funds_withdrawal
    expected="$(result insufficient_funds_withdrawal)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown account withdrawal" {
    run scoring unknown_account_withdrawal
    expected="$(result unknown_account_withdrawal)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "negative amount operations" {
    run scoring negative_amount_operations
    expected="$(result negative_amount_operations)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "duplicate operations" {
    run scoring duplicate_operations
    expected="$(result duplicate_operations)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute init success" {
    run scoring dispute_init_success
    expected="$(result dispute_init_success)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute already in progess" {
    run scoring dispute_already_in_progess
    expected="$(result dispute_already_in_progess)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute withdrawal" {
    run scoring dispute_withdrawal
    expected="$(result dispute_withdrawal)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute resolve success" {
    run -0 scoring dispute_resolve_success
    expected="$(result dispute_resolve_success)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown dispute resolve" {
    run -0 scoring unknown_dispute_resolve
    expected="$(result unknown_dispute_resolve)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown account dispute resolve" {
    run -0 scoring unknown_account_dispute_resolve
    expected="$(result unknown_account_dispute_resolve)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute chargeback success" {
    run -0 scoring dispute_chargeback_success
    expected="$(result dispute_chargeback_success)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown dispute chargeback" {
    run -0 scoring unknown_dispute_chargeback
    expected="$(result unknown_dispute_chargeback)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown account dispute chargeback" {
    run -0 scoring unknown_account_dispute_chargeback
    expected="$(result unknown_account_dispute_chargeback)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "amount rounding" {
    run -0 scoring amount_rounding
    expected="$(result amount_rounding)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute resolve resolve chargeback" {
    run -0 scoring dispute_resolve_resolve_chargeback
    expected="$(result dispute_resolve_resolve_chargeback)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute resolve resolve" {
    run -0 scoring dispute_resolve_resolve
    expected="$(result dispute_resolve_resolve)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "input csv amount column in the middle" {
    run -0 scoring input_csv_amount_column_in_the_middle
    expected="$(result input_csv_amount_column_in_the_middle)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}
