module adder(
  input logic [1: 0] a,
  input logic [1: 0] b,
  output logic [2: 0] s
);
  assign s = a + b;
endmodule
