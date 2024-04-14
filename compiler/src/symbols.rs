#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TapeSymbols {
    Blank,            // Default value for empty tape cells
    Zero,             // binary 0
    One,              // binary 1
    StartA,           // Start of the number A in the working area
    EndA,             // Seperator between numbers A and B in the working area
    EndB,             // End of the number B in the working area
    EndC,             // End of the number C in the working area
    Middle,           // Middle of the tape, between the storage area and the working area
    StorageSeperator, // Seperator between numbers/cells in the storage area
    HasMovedHelper0,  // Helper symbol to indicate that 0 bit has been moved
    HasMovedHelper1,  // Helper symbol to indicate that 1 bit has been moved
    StorageMarker,    // Marker to indicate which storage cell is being used
    MultiplyHelper,   // Helper symbol to indicate that the multiplication has been done on a digit
}

pub fn symtou8(symbol: TapeSymbols) -> u8 {
    match symbol {
        TapeSymbols::Zero => 0,
        TapeSymbols::One => 1,
        TapeSymbols::StartA => 2,
        TapeSymbols::EndA => 3,
        TapeSymbols::EndB => 4,
        TapeSymbols::Middle => 5,
        TapeSymbols::StorageSeperator => 6,
        TapeSymbols::HasMovedHelper0 => 7,
        TapeSymbols::HasMovedHelper1 => 8,
        TapeSymbols::StorageMarker => 9,
        TapeSymbols::EndC => 10,
        TapeSymbols::MultiplyHelper => 11,
        TapeSymbols::Blank => 255,
    }
}
