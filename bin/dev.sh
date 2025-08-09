if which concurrently >/dev/null; then
    date "+[%F %T] concurrently is 👍"
else
    date "+[%F %T] concurrently is not installed 🪲"
    cargo install concurrently
fi

if which stylance >/dev/null; then
    date "+[%F %T] stylance is ok 👍"
else
    date "+[%F %T] stylance is not installed 🪲"
    cargo install stylance-cli
fi

concurrently
