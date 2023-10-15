module popcount(
   input logic [3:0]  in,
  output logic [2:0] out
);
  assign out = in[3] + in[2] + in[1] + in[0];
endmodule
