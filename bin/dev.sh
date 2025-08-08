if which concurrently >/dev/null; then
    echo "concurrently is ok"
else
    echo "concurrently is not installed"
    cargo install concurrently
fi

if which stylance >/dev/null; then
    echo "stylance is ok"
else
    echo "stylance is not installed"
    cargo install stylance-cli
fi

concurrently