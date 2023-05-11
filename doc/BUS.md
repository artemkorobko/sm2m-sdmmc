# SM2M SDMMC Adapter Bus

This document describes data bus lines and their characteristics.

All input lines are 5v tolerant and capable of handling 5v levels. All output lines operate only on 3.3v level.

# Input BUS

| Line    | Abbreviation RUS | Description                         | Description RUS                                          |
| ------- | ---------------- | ----------------------------------- | -------------------------------------------------------- |
| DI_0    | ШИН0-И           | Data bus input bit 0                | Шина выдачи информации в устройство воода-вывода, бит 0  |
| DI_1    | ШИН1-И           | Data bus input bit 1                | Шина выдачи информации в устройство воода-вывода, бит 1  |
| DI_2    | ШИН2-И           | Data bus input bit 2                | Шина выдачи информации в устройство воода-вывода, бит 2  |
| DI_3    | ШИН3-И           | Data bus input bit 3                | Шина выдачи информации в устройство воода-вывода, бит 3  |
| DI_4    | ШИН4-И           | Data bus input bit 4                | Шина выдачи информации в устройство воода-вывода, бит 4  |
| DI_5    | ШИН5-И           | Data bus input bit 5                | Шина выдачи информации в устройство воода-вывода, бит 5  |
| DI_6    | ШИН6-И           | Data bus input bit 6                | Шина выдачи информации в устройство воода-вывода, бит 6  |
| DI_7    | ШИН7-И           | Data bus input bit 7                | Шина выдачи информации в устройство воода-вывода, бит 7  |
| DI_8    | ШИН8-И           | Data bus input bit 8                | Шина выдачи информации в устройство воода-вывода, бит 8  |
| DI_9    | ШИН9-И           | Data bus input bit 9                | Шина выдачи информации в устройство воода-вывода, бит 9  |
| DI_10   | ШИН10-И          | Data bus input bit 10               | Шина выдачи информации в устройство воода-вывода, бит 10 |
| DI_11   | ШИН11-И          | Data bus input bit 11               | Шина выдачи информации в устройство воода-вывода, бит 11 |
| DI_12   | ШИН12-И          | Data bus input bit 12               | Шина выдачи информации в устройство воода-вывода, бит 12 |
| DI_13   | ШИН13-И          | Data bus input bit 13               | Шина выдачи информации в устройство воода-вывода, бит 13 |
| DI_14   | ШИН14-И          | Data bus input bit 14               | Шина выдачи информации в устройство воода-вывода, бит 14 |
| DI_15   | ШИН15-И          | Data bus input bit 15               | Шина выдачи информации в устройство воода-вывода, бит 15 |
| CTRLI_0 | КР0-И            | Data bus control bit for bits 0..7  | Контрольный разряд 0 для битов 0..7                      |
| CTRLI_1 | КР1-И            | Data bus control bit for bits 8..15 | Контрольный разряд 1 для битов 8..15                     |
| RSTI    | ОСБ-И            | Reset signal                        | Сигнал сброса                                            |
| DTSI    | ВП-И             | Execute signal impulse              | Сигнал ВЫПОЛНИТЬ импульсный                              |
| DTLI    | ВП-ИП            | Execite signal state                | Сигнал ВЫПОЛНИТЬ потенциальный                           |
| DTEI    | ОСТ-ИП           | Data transfer end signal            | Сигнал останова передачи данных                          |

- `DI_0..DI_15` are plain data bit lines.
- `CTRLI_0` is set to 1 when number of bits in high byte set to 1 on data bit lines are even. Otherwise is set to 0.
- `CTRLI_1` is the same as `CTRLI_0` but for low byte.
- `RST` indicates that bus and all periferals connected to the bus should be set to their initial state.
- `DTSI` short signal which indicates that data transfer begins and adapter can start reading data and signal lines.
- `DTLI` unlike DTSI this signal lasts for 625ns with 3us delay after DI is set and used as interrupt source for adapter.
- `DTEI` a signal which indicates end of data transfer.

# Output BUS

| Line    | Abbreviation RUS | Description                         | Description RUS                                          |
| ------- | ---------------- | ----------------------------------- | -------------------------------------------------------- |
| DO_0    | ШИН0-П           | Data bus output bit 0               | Шина приема информации от устройства воода-вывода бит 0  |
| DO_1    | ШИН1-П           | Data bus output bit 1               | Шина приема информации от устройства воода-вывода бит 1  |
| DO_2    | ШИН2-П           | Data bus output bit 2               | Шина приема информации от устройства воода-вывода бит 2  |
| DO_3    | ШИН3-П           | Data bus output bit 3               | Шина приема информации от устройства воода-вывода бит 3  |
| DO_4    | ШИН4-П           | Data bus output bit 4               | Шина приема информации от устройства воода-вывода бит 4  |
| DO_5    | ШИН5-П           | Data bus output bit 5               | Шина приема информации от устройства воода-вывода бит 5  |
| DO_6    | ШИН6-П           | Data bus output bit 6               | Шина приема информации от устройства воода-вывода бит 6  |
| DO_7    | ШИН7-П           | Data bus output bit 7               | Шина приема информации от устройства воода-вывода бит 7  |
| DO_8    | ШИН8-П           | Data bus output bit 8               | Шина приема информации от устройства воода-вывода бит 8  |
| DO_9    | ШИН9-П           | Data bus output bit 9               | Шина приема информации от устройства воода-вывода бит 9  |
| DO_10   | ШИН10-П          | Data bus output bit 10              | Шина приема информации от устройства воода-вывода бит 10 |
| DO_11   | ШИН11-П          | Data bus output bit 11              | Шина приема информации от устройства воода-вывода бит 11 |
| DO_12   | ШИН12-П          | Data bus output bit 12              | Шина приема информации от устройства воода-вывода бит 12 |
| DO_13   | ШИН13-П          | Data bus output bit 13              | Шина приема информации от устройства воода-вывода бит 13 |
| DO_14   | ШИН14-П          | Data bus output bit 14              | Шина приема информации от устройства воода-вывода бит 14 |
| DO_15   | ШИН15-П          | Data bus output bit 15              | Шина приема информации от устройства воода-вывода бит 15 |
| CTRLO_0 | КР0-П            | Data bus control bit for bits 0..7  | Контрольный разряд 0 для битов 0..7                      |
| CTRLO_1 | КР1-П            | Data bus control bit for bits 8..15 | Контрольный разряд 1 для битов 8..15                     |
| RDY     | ГТ-П             | Ready signal                        | Сигнал готовности от устройства воода-вывода             |
| CTRLD   | ОК               | Enable/Disable CTRLO_x lines        | Включение/Выключение контрольных разрядов CTRLO_x        |
| ERRO    | ОШ               | Internal error                      | Сигнал индикации ошибки                                  |
| RSTE    | ВНС              | External reset signal               | Сигнал внешнего сброса                                   |
| SETE    | ВНУ              | External set signal                 | Сигнал внешней установки                                 |
| DTEO    | КОП              | Data transfer end signal            | Сигнал окончания передачи данных                         |

- `DO_0..DO_15` are plain data bit lines.
- `CTRLO_0` is set to 1 when number of bits in high byte set to 1 on data bit lines are even. Otherwise is set to 0.
- `CTRLO_1` is the same as `CTRLO_0` but for low byte.
- `RDY` signal indicates to SM2M that it can start reading bus lines.
- `CTRLD` if set to 1 disables verification of `CTRLO_0` and `CTRLO_1` lines by SM2M.
- `ERR` signal indicates adapter internal error and last data which adapter sent should be ignored.
- `RSTE` not used.
- `SETE` not used.
- `DTEO` signal notifies SM2M about the end of data transfer from adapter side.
