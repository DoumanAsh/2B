target remote :3333
set print asm-demangle on
set print pretty on

monitor arm semihosting enable
#monitor arm semihosting_fileio enable
#monitor tpiu config internal itm.log uart off 8000000
#monitor itm port 0 on

load
break main
continue
