#1/bin/sh

cargo run --features tools --release --bin generate_event_log -- \
    --accounts $ACCOUNTS --events $EVENTS --file $EVENT_LOG \
    1>/dev/null 2>/dev/null
