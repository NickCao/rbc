all: adder.pla popcount.pla adder.aag popcount.aag

%.aag : %.sv
	yosys -p "read_verilog -sv $^; aigmap; write_aiger -ascii -symbols $@"

%.aig : %.aag
	aigtoaig $^ $@

%.pla : %.aig
	yosys-abc -c "read $^; collapse; write $@"

clean:
	rm *.aag *.aig *.pla
