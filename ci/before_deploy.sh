# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    echo $src
    echo $stage

    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    cross rustc --bin sudoku --target $TARGET --release -- -C lto

    # TODO Update this to package the right artifacts
    case $TARGET in
      *windows*)
        cp target/$TARGET/release/sudoku.exe $stage/
        ;;
      *linux*)
        cp target/$TARGET/release/sudoku $stage/
        ;;
      *apple*)
        cp target/$TARGET/release/sudoku $stage/
        ;;
    esac
    test -d $stage/assets || mkdir -p $stage/assets && cp assets/FiraSans-Regular.ttf $stage/assets

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
