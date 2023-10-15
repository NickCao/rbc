all: adder.aig popcount.aig

%.aig : %.aag
	aigtoaig $^ $@

%.aag : %.sv
	yosys -p "read_verilog -sv $^; aigmap; write_aiger -ascii -symbols $@"
