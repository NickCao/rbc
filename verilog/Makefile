all: adder.aag popcount.aag

%.aag : %.sv
	yosys -p "read_verilog -sv $^; aigmap; write_aiger -ascii -symbols $@"
