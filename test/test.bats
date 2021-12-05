setup() {
    load 'test_helper/bats-support/load'
    load 'test_helper/bats-assert/load'

    CASES=./test/cases
}

command() {
    output="$(cargo run --release -- $CASES/$1/input.csv 2>/dev/null)"; assert_equal $? 0

    {
        echo "$output" | head -n 1
        echo "$output" | tail -n +2 | sort
    }
}

expected() {
    {
        head -n 1 $CASES/$1/output.csv
        tail -n +2 $CASES/$1/output.csv | sort
    }
}

@test "example" {
    run command example
    expected="$(expected example)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "basic successful operations single account" {
    run command basic_successful_operations_single_account
    expected="$(expected basic_successful_operations_single_account)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "basic successful operations multiple accounts" {
    run command basic_successful_operations_multiple_accounts
    expected="$(expected basic_successful_operations_multiple_accounts)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "insufficient funds withdrawal" {
    run command insufficient_funds_withdrawal
    expected="$(expected insufficient_funds_withdrawal)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown account withdrawal" {
    run command unknown_account_withdrawal
    expected="$(expected unknown_account_withdrawal)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "negative amount operations" {
    run command negative_amount_operations
    expected="$(expected negative_amount_operations)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "duplicate operations" {
    run command duplicate_operations
    expected="$(expected duplicate_operations)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute init success" {
    run command dispute_init_success
    expected="$(expected dispute_init_success)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute already in progess" {
    run command dispute_already_in_progess
    expected="$(expected dispute_already_in_progess)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute withdrawal" {
    run command dispute_withdrawal
    expected="$(expected dispute_withdrawal)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute resolve success" {
    run -0 command dispute_resolve_success
    expected="$(expected dispute_resolve_success)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown dispute resolve" {
    run -0 command unknown_dispute_resolve
    expected="$(expected unknown_dispute_resolve)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown account dispute resolve" {
    run -0 command unknown_account_dispute_resolve
    expected="$(expected unknown_account_dispute_resolve)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute chargeback success" {
    run -0 command dispute_chargeback_success
    expected="$(expected dispute_chargeback_success)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown dispute chargeback" {
    run -0 command unknown_dispute_chargeback
    expected="$(expected unknown_dispute_chargeback)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "unknown account dispute chargeback" {
    run -0 command unknown_account_dispute_chargeback
    expected="$(expected unknown_account_dispute_chargeback)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "amount rounding" {
    run -0 command amount_rounding
    expected="$(expected amount_rounding)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute resolve resolve chargeback" {
    run -0 command dispute_resolve_resolve_chargeback
    expected="$(expected dispute_resolve_resolve_chargeback)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "dispute resolve resolve" {
    run -0 command dispute_resolve_resolve
    expected="$(expected dispute_resolve_resolve)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}

@test "input csv amount column in the middle" {
    run -0 command input_csv_amount_column_in_the_middle
    expected="$(expected input_csv_amount_column_in_the_middle)"
    assert_output "$expected"; assert_not_equal "$expected" ""
}
