#!/bin/sh

cargo run --features tools --release -- $EVENT_LOG 1>/dev/null 2>/dev/null
