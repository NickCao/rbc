all: aag/adder.aag aag/popcount.aag

aag/%.aag : verilog/%.sv
	yosys -p "read_verilog -sv $^; proc; synth; aigmap; write_aiger -ascii -symbols $@"
