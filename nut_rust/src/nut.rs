pub struct NutProject {

}

/// Module is a high level entity of Nut program
pub struct NutModule {
    name: String,
}

pub struct NutFunction {
    // signature
    name: String,
    instructions: Vec<NutInstructionCode>,
}

/// Prototype
pub struct NutPrototype {
    name: String,
    args: Vec<String>
}

pub struct NutItem {
    /// ref to module
    item_type: NutType,
    item_value: ItemValue,
}

pub struct NutStr {
    len: u8,
    string: String
}

pub struct NutName {
    name: String
}

pub struct ItemValue {
    func: NutFunction,
    proto: NutPrototype,
    import_id: NutName,
    export_id: NutName,
    forward_id: NutName,
    // todo: data
    data: String,
    // todo: ref data
    ref_data: String,
    // todo: expr data
    expr_data: String,
}

/// MIR_insn_code_t
/// Insns: [GNU Insns](https://gcc.gnu.org/onlinedocs/gccint/Insns.html)
pub enum NutInstructionCode {
    // 2 operand insns: */
    Moves,
    // todo: Extensions
    // 3. operand insn
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
    Call(NutCall),
}

pub struct NutCall {
    prototype: String,
    inline: String
}

pub struct NutInline {

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

pub enum NutIntegerType {
    I8, U8, I16, U16, I32, U32, I64, U64
}

pub enum NutFloatType {
    Float, Double, LongDouble,
}

pub enum NutPointerType {
    Pointer, MemoryBlock
}
pub enum NutReturnBlock {}
pub enum NutCollection {
    Undef, Bound
}
