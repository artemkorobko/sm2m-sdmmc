# SM2M SDMMC Adapter Bus

This document describes data bus lines and their characteristics.

All input lines are 5v tolerant and capable of handling 5v levels. All output lines operate only on 3.3v level.

# Input line

| Bit | Line    | Abbreviation RUS | Description                         |
| --: | ------- | ---------------- | ----------------------------------- |
| 0   | DI_0    | ШИН0-И           | Data bus input bit 0                |
| 1   | DI_1    | ШИН1-И           | Data bus input bit 1                |
| 2   | DI_2    | ШИН2-И           | Data bus input bit 2                |
| 3   | DI_3    | ШИН3-И           | Data bus input bit 3                |
| 4   | DI_4    | ШИН4-И           | Data bus input bit 4                |
| 5   | DI_5    | ШИН5-И           | Data bus input bit 5                |
| 6   | DI_6    | ШИН6-И           | Data bus input bit 6                |
| 7   | DI_7    | ШИН7-И           | Data bus input bit 7                |
| 8   | DI_8    | ШИН8-И           | Data bus input bit 8                |
| 9   | DI_9    | ШИН9-И           | Data bus input bit 9                |
| 10  | DI_10   | ШИН10-И          | Data bus input bit 10               |
| 11  | DI_11   | ШИН11-И          | Data bus input bit 11               |
| 12  | DI_12   | ШИН12-И          | Data bus input bit 12               |
| 13  | DI_13   | ШИН13-И          | Data bus input bit 13               |
| 14  | DI_14   | ШИН14-И          | Data bus input bit 14               |
| 15  | DI_15   | ШИН15-И          | Data bus input bit 15               |
| 16  | CTRLI_0 | КР0-И            | Data bus control bit for bits 0..7  |
| 17  | CTRLI_1 | КР1-И            | Data bus control bit for bits 8..15 |
| 18  | RSTI    | ОСБ-И            | Data bus reset signal               |
| 19  | DTSI    | ВП-И             | Data transfer begin short signal    |
| 20  | DTLI    | ВП-ИП            | Data transfer begin long signal     |
| 21  | DTEI    | ОСТ-ИП           | Data transfer end signal            |

- `DI_0..DI_15` are plain data bit lines.
- `CTRLI_0` is set to 1 when number of bits in high byte set to 1 on data bit lines are even. Otherwise is set to 0.
- `CTRLI_1` is the same as `CTRLI_0` but for low byte.
- `RST` indicates that bus and all periferals connected to the bus should be set to their initial state.
- `DTSI` short signal which indicates that data transfer begins and adapter can start reading data and signal lines.
- `DTLI` unlike DTSI this signal lasts for entire request-reply act and used as interrupt source for adapter.
- `DTEI` a signal which indicates end of data transfer.

# Output line

| Bit | Line    | Abbreviation RUS | Description                         |
| --: | ------- | ---------------- | ----------------------------------- |
| 0   | DO_0    | ШИН0-П           | Data bus output bit 0               |
| 1   | DO_1    | ШИН1-П           | Data bus output bit 1               |
| 2   | DO_2    | ШИН2-П           | Data bus output bit 2               |
| 3   | DO_3    | ШИН3-П           | Data bus output bit 3               |
| 4   | DO_4    | ШИН4-П           | Data bus output bit 4               |
| 5   | DO_5    | ШИН5-П           | Data bus output bit 5               |
| 6   | DO_6    | ШИН6-П           | Data bus output bit 6               |
| 7   | DO_7    | ШИН7-П           | Data bus output bit 7               |
| 8   | DO_8    | ШИН8-П           | Data bus output bit 8               |
| 9   | DO_9    | ШИН9-П           | Data bus output bit 9               |
| 10  | DO_10   | ШИН10-П          | Data bus output bit 10              |
| 11  | DO_11   | ШИН11-П          | Data bus output bit 11              |
| 12  | DO_12   | ШИН12-П          | Data bus output bit 12              |
| 13  | DO_13   | ШИН13-П          | Data bus output bit 13              |
| 14  | DO_14   | ШИН14-П          | Data bus output bit 14              |
| 15  | DO_15   | ШИН15-П          | Data bus output bit 15              |
| 16  | CTRLO_0 | КР0-П            | Data bus control bit for bits 0..7  |
| 17  | CTRLO_1 | КР1-П            | Data bus control bit for bits 8..15 |
| 18  | RDY     | ГТ-П             | Ready signal                        |
| 19  | CTRLD   | ОК               | Disable CTRLO_x lines               |
| 20  | ERRO    | ОШ               | Internal error                      |
| 21  | RSTE    | ВНС              | External reset signal               |
| 22  | SETE    | ВНУ              | External set signal                 |
| 23  | DTEO    | КОП              | Data transfer end signal            |

- `DO_0..DO_15` are plain data bit lines.
- `CTRLO_0` is set to 1 when number of bits in high byte set to 1 on data bit lines are even. Otherwise is set to 0.
- `CTRLO_1` is the same as `CTRLO_0` but for low byte.
- `RDY` signal indicates to SM2M that it can start reading bus lines.
- `CTRLD` if set to 1 disables verification of `CTRLO_0` and `CTRLO_1` lines by SM2M.
- `ERR` signal indicates adapter internal error and last data which adapter sent should be ignored.
- `RSTE` not used.
- `SETE` not used.
- `DTEO` signal notifies SM2M about the end of data transfer from adapter side.
