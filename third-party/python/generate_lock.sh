SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

bazelisk run //third-party/python:requirements-x86_64_linux.update
sed -i'bkp' '1s/^/--platform manylinux2014_x86_64\n/' $SCRIPT_DIR/requirements-x86_64_linux.txt

bazelisk run //third-party/python:requirements-aarch64_linux.update
sed -i'bkp' '1s/^/--platform manylinux2014_aarch64\n/' $SCRIPT_DIR/requirements-aarch64_linux.txt

# No extra platform arguments for macOS
bazelisk run //third-party/python:requirements-x86_64_macos.update
bazelisk run //third-party/python:requirements-aarch64_macos.update
