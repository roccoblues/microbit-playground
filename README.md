# microbit-playground

Examples programming the [micro:bit](https://microbit.org/). The initial setup was done with the awesome [knurling-rs/app-template](https://github.com/knurling-rs/app-template).

## [temp](src/bin/temp.rs)

Read the nRF52 temperature sensor value.

```
> cargo rb temp
Temperature: 26.2
└─ temp::__cortex_m_rt_main @ src/bin/temp.rs:13
```

## [balance](src/bin/balance.rs)

Uses the accelerometer on the micro:bit to "balance" a led on the board. Flash with `cargo fl balance`.
