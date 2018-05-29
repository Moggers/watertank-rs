openocd:
	openocd -f "interface/cmsis-dap.cfg" -f ./rtl8710.ocd -c "init" -c "reset halt"
