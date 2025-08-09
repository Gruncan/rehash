
if [ -z "$1" ]; then
  # shellcheck disable=SC2162
  read -p "Enter build target [debug/release]: " input
else
  input=$1
fi


case "$input" in
  release )
    target="--release"
    ;;
  * )
    target="--debug"
        ;;
esac

wasm-pack build --target web --out-dir ../../rehash-desktop/pkg $target
