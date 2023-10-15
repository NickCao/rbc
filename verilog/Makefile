all: adder.pla popcount.pla adder.aag popcount.aag

%.aig : %.sv
	yosys -p "read_verilog -sv $^; synth; aigmap; write_aiger -symbols $@"

%.aag : %.aig
	aigtoaig $^ $@

%.pla : %.aig
	yosys-abc -c "read $^; collapse; write $@"

clean:
	rm *.aag *.aig *.pla
