Design document of a compiler to turingcode with a u8 tape the turingcode looks like [CURRENT_STATE, READ_VALUE, WRITE_VALUE, MOVE_DIRECTION, NEXT_STATE]

# Symbols
[Symbols](src\symbols.rs)

# Turing Machine Tape Segments
__ [Storage Area] 5 [Working Area] __

# Turing Machine Working Area Tape
[Storage Area]5 __ 2[A]3[B]4 __

# Turing Machine Storage Area Tape
__ ...6[S3]6[S2]6[S1]6 __ 5[Working Area]

# Full Tape Example
__ ...6[S3]6[S2]6[S1]6 5 2[A]3[B]4 __

# Working Area
The working area only ever contains two variables, A and B. The working area is used for calculations and temporary storage of values.

# Storage Area
The storage area can store an infinite number of variables to the left of the working area. The storage area is divided into individual cells, each cell is 8 bits long and can store an u8. To store a variable in the storage area, the STORE instruction is used, which takes the variable from the working area and a storage area cell index as arguments. To refer to a cell index use S followed by the index number, for example S1, S2, S3, etc.
At the end of the program, the result is stored in the first cell S1 of the storage area.

# Intermediary Assembly Language Operations
- SAVE value, S(i): Save the value directly into the storage cell S(i). This operation edits the Starting tape, and does not add any instructions.

- LOAD S(i), (A/B): Load the value from the storage cell S(i) into the working area variable A or B. This operation copies the value, so the original value in the storage cell is not modified.

- STORE (A/B), S(i): Store the value from the working area variable A or B into the storage cell S(i). This operation copies the value, so the original value in the working area variable is not modified.

- MOVE S(i), S(j): Move the value from storage cell S(i) to storage cell S(j).
- ADD A, B: Add the values in the working area variables A and B, store the result in A.
- SUB A, B: Subtract the values in the working area variables A and B, store the result in A.
- MUL A, B: Multiply the values in the working area variables A and B, store the result in A.


# Starting Code
5 + 6 * 7

# TAC (Three Address Code)
t1 = 5
t2 = 6
t3 = 7
t4 = t2 * t3
t5 = t1 + t4


# Intermediary Assembly Language Example
; Example Assembly Language for a Turing Machine Compiler

; Define start of program
START

; Save all values in the storage area
SAVE 5, S1
SAVE 6, S2 
SAVE 7, S3 

; Load values into working area
LOAD S1, A
LOAD S2, B 

; Perform a multiplication
MUL A, B        ; Multiply A and B, result in A

; Store the result from A into the third position of the storage area
STORE A, S4 

; Load new values into working area for addition
LOAD S3, A
LOAD S4, B

; Perform an addition
ADD A, B        ; Add A and B, result in A

; Store result
STORE A, S5      ; Store the result from A into the first position of the storage area

; Move S5 to S1. This is the final result
MOVE S5, S1
