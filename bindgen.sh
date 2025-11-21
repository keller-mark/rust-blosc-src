#! /bin/sh

BLOSC_DIR="c-blosc"

#export OPTIMIZE="-Os -flto"
#export LDFLAGS=$OPTIMIZE
#export CFLAGS=$OPTIMIZE
#export CPPFLAGS=$OPTIMIZE


# Add missing headers in vendored zlib.
for file in "gzlib.c" "gzread.c" "gzwrite.c"; do \
  # macOS sed command
  #sed -i '' "1s/^/#include <unistd.h>/" "$BLOSC_DIR/internal-complibs/zlib-1.3.1/$file" ; \
  # Linux sed command
  sed -i "1s/^/#include <unistd.h>/" "$BLOSC_DIR/internal-complibs/zlib-1.3.1/$file" ; \

  # TODO: update this so that it does not add more than one include, when run multiple times.
done

echo "============================================="
echo "Compiling blosc"
echo "============================================="

# AVX2 and SSE2 are not supported in WebAssembly
bindgen "$BLOSC_DIR/blosc/blosc.h" \
	-o src/bindgen.rs \
	--no-rustfmt-bindings \
	--blocklist-type __uint64_t \
	--blocklist-type __size_t \
	--allowlist-type '.*BLOSC.*' \
	--allowlist-function '.*blosc.*' \
	--allowlist-var '.*BLOSC.*' \
	--clang-macro-fallback \
	--rust-target 1.64 \
	-- \
	-I"$BLOSC_DIR/blosc" \
	-x c++ \
	-std=c++17 \
	-DDEACTIVATE_AVX2=ON \
	-DDEACTIVATE_SSE2=ON
rustfmt src/bindgen.rs
