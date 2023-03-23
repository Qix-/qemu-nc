# qemu-nc

A dumb netcat for QEMU's `-netdev socket` protocol.

**tl;dr** the format is `<u32be: length><ethernet frame data>`
sent as a stream of bytes over the wire. Nothing more.

## Why?

I'm developing an operating system and wanted to be
able to check that implementations of the network
are working correctly. To do that, I'd need to test
the format of ethernet packets coming out of the machine
under QEMU in a generic way so that QEMU can be added as
a kernel/system integration test backend along with some
custom PCBs used for testing baremetal setups.

I didn't want to have to extend QEMU (due to equal parts
time, complexity, and licensing) so instead looked for
other ways to instrument the network of the SUT. This appears
to be the most generic, low-level, "raw" way to do so.

I have not tried to send packets back over the wire
but I would imagine it works the same.

## Which operating system?

[This one.](https://github.com/oro-os)

## Usage

Build, run. It'll give you a port (it's _not_ the same each time).

Run QEMU with something like the following:

```shell
qemu-system-x86_64 -nographic -netdev socket,id=n1,connect=127.0.0.1:$PORT -device e1000,netdev=n1,mac=52:54:00:12:34:56
```

You should start seeing packets come across `qemu-nc`.

# License

Copyright &copy; 2023, Josh Junon.
Licensed under the [WTFPL](LICENSE). There's not a
lot of code here anyway, so do whatever you want with it.
Treat it as CC0 or PD if you want, I really honestly don't care.
