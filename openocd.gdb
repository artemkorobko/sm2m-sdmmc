target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

monitor arm semihosting enable
load

# send captured ITM to the file itm.fifo
# (the microcontroller SWO pin must be connected to the programmer SWO pin)
# 72000000 must match the core clock frequency
monitor tpiu config internal itm.txt uart off 72000000

# start the process but immediately halt the processor
stepi
