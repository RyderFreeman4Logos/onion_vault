// mod trezor;



pub fn print_errs_and_solution() {
    println!(r#"
Trezor Error: Ensure that udev and libusb is set up,
|-- then unplug and replug your Trezor and enter the Trezor boot password.
|----- ueful links:
|---------- https://trezor.io/learn/a/udev-rules
|---------- https://trezor.io/learn/a/trezorctl-on-macos

        "#);
}
