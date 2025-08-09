
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


cd components || exit 1

cd rehash-frontend || exit 1
./build-frontend.sh $target

cd ../rehash-loader || exit 1
./build-loader.sh $target
