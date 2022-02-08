pub struct NutModule {
    name: String,
}

pub struct NutFunctions {
    // signature
    name: String,
    instructions: Vec<NutInstructionCode>,
}

pub struct NutProto {
    name: String,
    args: Vec<String>
}

pub struct NutItem {
    /// ref to module
    item_type: NutType,
}

pub struct NutStr {
    len: u8,
    string: String
}

/// MIR_insn_code_t
/// Insns: [GNU Insns](https://gcc.gnu.org/onlinedocs/gccint/Insns.html)
pub enum NutInstructionCode {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Logical,
    RightSigned,
    Equality,
    Inequality,
    LessThen,
    LessOrEqual,
    GreaterThen,
    GreaterOrEqual,
    // todo: others
}

pub enum NutType {
    /// Integer types of different size:
    IntegerType(NutIntegerType),
    /// Float or (long) double type
    FloatType(NutFloatType),
    /// Pointer, memory block
    PointerType(NutPointerType),
    /// return block
    TypeType(NutReturnBlock),
    /// UNDEF, BOUND ? may be array
    CollectionType(NutCollection)
}

pub enum NutIntegerType {}
pub enum NutFloatType {}
pub enum NutPointerType {}
pub enum NutReturnBlock {}
pub enum NutCollection {}
