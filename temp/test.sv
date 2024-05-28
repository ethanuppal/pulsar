module undef #(
    parameter WIDTH = 32
) (
   output logic [WIDTH-1:0] out
);
assign out = 'x;
endmodule

module std_const #(
    parameter WIDTH = 32,
    parameter VALUE = 32
) (
   output logic [WIDTH-1:0] out
);
assign out = VALUE;
endmodule

module std_wire #(
    parameter WIDTH = 32
) (
   input logic [WIDTH-1:0] in,
   output logic [WIDTH-1:0] out
);
assign out = in;
endmodule

module std_add #(
    parameter WIDTH = 32
) (
   input logic [WIDTH-1:0] left,
   input logic [WIDTH-1:0] right,
   output logic [WIDTH-1:0] out
);
assign out = left + right;
endmodule

module std_reg #(
    parameter WIDTH = 32
) (
   input logic [WIDTH-1:0] in,
   input logic write_en,
   input logic clk,
   input logic reset,
   output logic [WIDTH-1:0] out,
   output logic done
);
always_ff @(posedge clk) begin
    if (reset) begin
       out <= 0;
       done <= 0;
    end else if (write_en) begin
      out <= in;
      done <= 1'd1;
    end else done <= 1'd0;
  end
endmodule

module _pulsar_Sinc_q_q(
  input logic [63:0] arg0,
  output logic [63:0] ret,
  input logic go,
  input logic clk,
  input logic reset,
  output logic done
);
// COMPONENT START: _pulsar_Sinc_q_q
logic [63:0] t0_in;
logic t0_write_en;
logic t0_clk;
logic t0_reset;
logic [63:0] t0_out;
logic t0_done;
logic [63:0] i0_in;
logic i0_write_en;
logic i0_clk;
logic i0_reset;
logic [63:0] i0_out;
logic i0_done;
logic [63:0] i1_in;
logic i1_write_en;
logic i1_clk;
logic i1_reset;
logic [63:0] i1_out;
logic i1_done;
logic [63:0] adder_left;
logic [63:0] adder_right;
logic [63:0] adder_out;
logic [1:0] fsm_in;
logic fsm_write_en;
logic fsm_clk;
logic fsm_reset;
logic [1:0] fsm_out;
logic fsm_done;
logic [1:0] adder0_left;
logic [1:0] adder0_right;
logic [1:0] adder0_out;
logic sig_reg_in;
logic sig_reg_write_en;
logic sig_reg_clk;
logic sig_reg_reset;
logic sig_reg_out;
logic sig_reg_done;
std_reg # (
    .WIDTH(64)
) t0 (
    .clk(t0_clk),
    .done(t0_done),
    .in(t0_in),
    .out(t0_out),
    .reset(t0_reset),
    .write_en(t0_write_en)
);
std_reg # (
    .WIDTH(64)
) i0 (
    .clk(i0_clk),
    .done(i0_done),
    .in(i0_in),
    .out(i0_out),
    .reset(i0_reset),
    .write_en(i0_write_en)
);
std_reg # (
    .WIDTH(64)
) i1 (
    .clk(i1_clk),
    .done(i1_done),
    .in(i1_in),
    .out(i1_out),
    .reset(i1_reset),
    .write_en(i1_write_en)
);
std_add # (
    .WIDTH(64)
) adder (
    .left(adder_left),
    .out(adder_out),
    .right(adder_right)
);
std_reg # (
    .WIDTH(2)
) fsm (
    .clk(fsm_clk),
    .done(fsm_done),
    .in(fsm_in),
    .out(fsm_out),
    .reset(fsm_reset),
    .write_en(fsm_write_en)
);
std_add # (
    .WIDTH(2)
) adder0 (
    .left(adder0_left),
    .out(adder0_out),
    .right(adder0_right)
);
std_reg # (
    .WIDTH(1)
) sig_reg (
    .clk(sig_reg_clk),
    .done(sig_reg_done),
    .in(sig_reg_in),
    .out(sig_reg_out),
    .reset(sig_reg_reset),
    .write_en(sig_reg_write_en)
);
wire _guard0 = 1;
wire _guard1 = go;
wire _guard2 = fsm_out == 2'd0;
wire _guard3 = _guard1 & _guard2;
wire _guard4 = fsm_out >= 2'd1;
wire _guard5 = fsm_out < 2'd3;
wire _guard6 = _guard4 & _guard5;
wire _guard7 = _guard3 | _guard6;
wire _guard8 = go;
wire _guard9 = fsm_out == 2'd0;
wire _guard10 = _guard8 & _guard9;
wire _guard11 = _guard7 & _guard10;
wire _guard12 = go;
wire _guard13 = fsm_out == 2'd0;
wire _guard14 = _guard12 & _guard13;
wire _guard15 = fsm_out >= 2'd1;
wire _guard16 = fsm_out < 2'd3;
wire _guard17 = _guard15 & _guard16;
wire _guard18 = _guard14 | _guard17;
wire _guard19 = go;
wire _guard20 = fsm_out == 2'd0;
wire _guard21 = _guard19 & _guard20;
wire _guard22 = _guard18 & _guard21;
wire _guard23 = fsm_out == 2'd0;
wire _guard24 = sig_reg_out;
wire _guard25 = _guard23 & _guard24;
wire _guard26 = go;
wire _guard27 = fsm_out == 2'd0;
wire _guard28 = _guard26 & _guard27;
wire _guard29 = fsm_out >= 2'd1;
wire _guard30 = fsm_out < 2'd3;
wire _guard31 = _guard29 & _guard30;
wire _guard32 = _guard28 | _guard31;
wire _guard33 = fsm_out == 2'd1;
wire _guard34 = _guard32 & _guard33;
wire _guard35 = go;
wire _guard36 = fsm_out == 2'd0;
wire _guard37 = _guard35 & _guard36;
wire _guard38 = fsm_out >= 2'd1;
wire _guard39 = fsm_out < 2'd3;
wire _guard40 = _guard38 & _guard39;
wire _guard41 = _guard37 | _guard40;
wire _guard42 = fsm_out == 2'd1;
wire _guard43 = _guard41 & _guard42;
wire _guard44 = go;
wire _guard45 = fsm_out == 2'd0;
wire _guard46 = _guard44 & _guard45;
wire _guard47 = fsm_out == 2'd2;
wire _guard48 = fsm_out != 2'd0;
wire _guard49 = fsm_out != 2'd2;
wire _guard50 = _guard48 & _guard49;
wire _guard51 = go;
wire _guard52 = fsm_out == 2'd0;
wire _guard53 = _guard51 & _guard52;
wire _guard54 = fsm_out >= 2'd1;
wire _guard55 = fsm_out < 2'd3;
wire _guard56 = _guard54 & _guard55;
wire _guard57 = _guard53 | _guard56;
wire _guard58 = fsm_out == 2'd2;
wire _guard59 = _guard57 & _guard58;
wire _guard60 = go;
wire _guard61 = fsm_out == 2'd0;
wire _guard62 = _guard60 & _guard61;
wire _guard63 = fsm_out >= 2'd1;
wire _guard64 = fsm_out < 2'd3;
wire _guard65 = _guard63 & _guard64;
wire _guard66 = _guard62 | _guard65;
wire _guard67 = fsm_out == 2'd2;
wire _guard68 = _guard66 & _guard67;
wire _guard69 = go;
wire _guard70 = fsm_out == 2'd0;
wire _guard71 = _guard69 & _guard70;
wire _guard72 = fsm_out >= 2'd1;
wire _guard73 = fsm_out < 2'd3;
wire _guard74 = _guard72 & _guard73;
wire _guard75 = _guard71 | _guard74;
wire _guard76 = fsm_out == 2'd1;
wire _guard77 = _guard75 & _guard76;
wire _guard78 = go;
wire _guard79 = fsm_out == 2'd0;
wire _guard80 = _guard78 & _guard79;
wire _guard81 = fsm_out >= 2'd1;
wire _guard82 = fsm_out < 2'd3;
wire _guard83 = _guard81 & _guard82;
wire _guard84 = _guard80 | _guard83;
wire _guard85 = fsm_out == 2'd1;
wire _guard86 = _guard84 & _guard85;
wire _guard87 = fsm_out == 2'd0;
wire _guard88 = go;
wire _guard89 = go;
wire _guard90 = ~_guard89;
assign i0_write_en = _guard11;
assign i0_clk = clk;
assign i0_reset = reset;
assign i0_in = arg0;
assign done = _guard25;
assign ret = t0_out;
assign adder_left = i0_out;
assign adder_right = 64'd1;
assign fsm_write_en = 1'd1;
assign fsm_clk = clk;
assign fsm_reset = reset;
assign fsm_in =
  _guard46 ? 2'd1 :
  _guard47 ? 2'd0 :
  _guard50 ? adder0_out :
  2'd0;
assign adder0_left = fsm_out;
assign adder0_right = 2'd1;
assign t0_write_en = _guard59;
assign t0_clk = clk;
assign t0_reset = reset;
assign t0_in = i1_out;
assign i1_write_en = _guard77;
assign i1_clk = clk;
assign i1_reset = reset;
assign i1_in = adder_out;
assign sig_reg_write_en = _guard87;
assign sig_reg_clk = clk;
assign sig_reg_reset = reset;
assign sig_reg_in =
  _guard88 ? 1'd1 :
  _guard90 ? 1'd0 :
  1'd0;
// COMPONENT END: _pulsar_Sinc_q_q
endmodule
module _pulsar_Smain_q_q(
  input logic [63:0] arg0,
  output logic [63:0] ret,
  input logic go,
  input logic clk,
  input logic reset,
  output logic done
);
// COMPONENT START: _pulsar_Smain_q_q
logic [63:0] t1_in;
logic t1_write_en;
logic t1_clk;
logic t1_reset;
logic [63:0] t1_out;
logic t1_done;
logic [63:0] i2_in;
logic i2_write_en;
logic i2_clk;
logic i2_reset;
logic [63:0] i2_out;
logic i2_done;
logic [63:0] call_pulsar_Sinc_q_q_arg0;
logic [63:0] call_pulsar_Sinc_q_q_ret;
logic call_pulsar_Sinc_q_q_go;
logic call_pulsar_Sinc_q_q_done;
logic call_pulsar_Sinc_q_q_clk;
logic call_pulsar_Sinc_q_q_reset;
logic [2:0] fsm_in;
logic fsm_write_en;
logic fsm_clk;
logic fsm_reset;
logic [2:0] fsm_out;
logic fsm_done;
logic [2:0] adder_left;
logic [2:0] adder_right;
logic [2:0] adder_out;
logic sig_reg_in;
logic sig_reg_write_en;
logic sig_reg_clk;
logic sig_reg_reset;
logic sig_reg_out;
logic sig_reg_done;
std_reg # (
    .WIDTH(64)
) t1 (
    .clk(t1_clk),
    .done(t1_done),
    .in(t1_in),
    .out(t1_out),
    .reset(t1_reset),
    .write_en(t1_write_en)
);
std_reg # (
    .WIDTH(64)
) i2 (
    .clk(i2_clk),
    .done(i2_done),
    .in(i2_in),
    .out(i2_out),
    .reset(i2_reset),
    .write_en(i2_write_en)
);
_pulsar_Sinc_q_q call_pulsar_Sinc_q_q (
    .arg0(call_pulsar_Sinc_q_q_arg0),
    .clk(call_pulsar_Sinc_q_q_clk),
    .done(call_pulsar_Sinc_q_q_done),
    .go(call_pulsar_Sinc_q_q_go),
    .reset(call_pulsar_Sinc_q_q_reset),
    .ret(call_pulsar_Sinc_q_q_ret)
);
std_reg # (
    .WIDTH(3)
) fsm (
    .clk(fsm_clk),
    .done(fsm_done),
    .in(fsm_in),
    .out(fsm_out),
    .reset(fsm_reset),
    .write_en(fsm_write_en)
);
std_add # (
    .WIDTH(3)
) adder (
    .left(adder_left),
    .out(adder_out),
    .right(adder_right)
);
std_reg # (
    .WIDTH(1)
) sig_reg (
    .clk(sig_reg_clk),
    .done(sig_reg_done),
    .in(sig_reg_in),
    .out(sig_reg_out),
    .reset(sig_reg_reset),
    .write_en(sig_reg_write_en)
);
wire _guard0 = 1;
wire _guard1 = fsm_out == 3'd0;
wire _guard2 = sig_reg_out;
wire _guard3 = _guard1 & _guard2;
wire _guard4 = go;
wire _guard5 = fsm_out == 3'd0;
wire _guard6 = _guard4 & _guard5;
wire _guard7 = fsm_out >= 3'd1;
wire _guard8 = fsm_out < 3'd6;
wire _guard9 = _guard7 & _guard8;
wire _guard10 = _guard6 | _guard9;
wire _guard11 = go;
wire _guard12 = fsm_out == 3'd0;
wire _guard13 = _guard11 & _guard12;
wire _guard14 = fsm_out == 3'd4;
wire _guard15 = _guard13 | _guard14;
wire _guard16 = _guard10 & _guard15;
wire _guard17 = go;
wire _guard18 = fsm_out == 3'd0;
wire _guard19 = _guard17 & _guard18;
wire _guard20 = fsm_out >= 3'd1;
wire _guard21 = fsm_out < 3'd6;
wire _guard22 = _guard20 & _guard21;
wire _guard23 = _guard19 | _guard22;
wire _guard24 = go;
wire _guard25 = fsm_out == 3'd0;
wire _guard26 = _guard24 & _guard25;
wire _guard27 = _guard23 & _guard26;
wire _guard28 = go;
wire _guard29 = fsm_out == 3'd0;
wire _guard30 = _guard28 & _guard29;
wire _guard31 = fsm_out >= 3'd1;
wire _guard32 = fsm_out < 3'd6;
wire _guard33 = _guard31 & _guard32;
wire _guard34 = _guard30 | _guard33;
wire _guard35 = fsm_out == 3'd4;
wire _guard36 = _guard34 & _guard35;
wire _guard37 = fsm_out != 3'd0;
wire _guard38 = fsm_out != 3'd5;
wire _guard39 = _guard37 & _guard38;
wire _guard40 = fsm_out == 3'd5;
wire _guard41 = go;
wire _guard42 = fsm_out == 3'd0;
wire _guard43 = _guard41 & _guard42;
wire _guard44 = go;
wire _guard45 = fsm_out == 3'd0;
wire _guard46 = _guard44 & _guard45;
wire _guard47 = fsm_out >= 3'd1;
wire _guard48 = fsm_out < 3'd6;
wire _guard49 = _guard47 & _guard48;
wire _guard50 = _guard46 | _guard49;
wire _guard51 = fsm_out >= 3'd1;
wire _guard52 = fsm_out < 3'd4;
wire _guard53 = _guard51 & _guard52;
wire _guard54 = _guard50 & _guard53;
wire _guard55 = go;
wire _guard56 = fsm_out == 3'd0;
wire _guard57 = _guard55 & _guard56;
wire _guard58 = fsm_out >= 3'd1;
wire _guard59 = fsm_out < 3'd6;
wire _guard60 = _guard58 & _guard59;
wire _guard61 = _guard57 | _guard60;
wire _guard62 = fsm_out >= 3'd1;
wire _guard63 = fsm_out < 3'd4;
wire _guard64 = _guard62 & _guard63;
wire _guard65 = _guard61 & _guard64;
wire _guard66 = go;
wire _guard67 = fsm_out == 3'd0;
wire _guard68 = _guard66 & _guard67;
wire _guard69 = fsm_out >= 3'd1;
wire _guard70 = fsm_out < 3'd6;
wire _guard71 = _guard69 & _guard70;
wire _guard72 = _guard68 | _guard71;
wire _guard73 = fsm_out == 3'd5;
wire _guard74 = _guard72 & _guard73;
wire _guard75 = go;
wire _guard76 = fsm_out == 3'd0;
wire _guard77 = _guard75 & _guard76;
wire _guard78 = fsm_out >= 3'd1;
wire _guard79 = fsm_out < 3'd6;
wire _guard80 = _guard78 & _guard79;
wire _guard81 = _guard77 | _guard80;
wire _guard82 = fsm_out == 3'd5;
wire _guard83 = _guard81 & _guard82;
wire _guard84 = fsm_out == 3'd0;
wire _guard85 = go;
wire _guard86 = go;
wire _guard87 = ~_guard86;
assign done = _guard3;
assign ret = t1_out;
assign adder_left = fsm_out;
assign adder_right = 3'd1;
assign i2_write_en = _guard16;
assign i2_clk = clk;
assign i2_reset = reset;
assign i2_in =
  _guard27 ? arg0 :
  _guard36 ? call_pulsar_Sinc_q_q_ret :
  'x;
assign fsm_write_en = 1'd1;
assign fsm_clk = clk;
assign fsm_reset = reset;
assign fsm_in =
  _guard39 ? adder_out :
  _guard40 ? 3'd0 :
  _guard43 ? 3'd1 :
  3'd0;
assign call_pulsar_Sinc_q_q_clk = clk;
assign call_pulsar_Sinc_q_q_arg0 =
  _guard54 ? i2_out :
  64'd0;
assign call_pulsar_Sinc_q_q_go = _guard65;
assign call_pulsar_Sinc_q_q_reset = reset;
assign t1_write_en = _guard74;
assign t1_clk = clk;
assign t1_reset = reset;
assign t1_in = i2_out;
assign sig_reg_write_en = _guard84;
assign sig_reg_clk = clk;
assign sig_reg_reset = reset;
assign sig_reg_in =
  _guard85 ? 1'd1 :
  _guard87 ? 1'd0 :
  1'd0;
// COMPONENT END: _pulsar_Smain_q_q
endmodule
