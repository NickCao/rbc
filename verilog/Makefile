all: adder.pla popcount.pla

clean:
	rm *.aag *.aig *.pla

%.aag : %.sv
	yosys -p "read_verilog -sv $^; aigmap; write_aiger -ascii -symbols $@"

%.aig : %.aag
	aigtoaig $^ $@

%.pla : %.aig
	yosys-abc -c "read $^; collapse; write $@"
