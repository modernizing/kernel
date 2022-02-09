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

///
/// ```java
///
/// ```
pub struct NutImport {
    pub source: String,
    pub as_name: String,
    pub usage_name: String,
    pub scope: String
}

/// parameter
pub struct NutParam {
    flag: ParamFlag
}

pub enum ParamFlag {
    In, Out, Retval
}

pub struct NutProperty {

}

pub enum NutStructModifier {
    Virtual,
    New,
    Override,
    Sealed,
    Abstract
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

/// design for prototype
pub struct NutCall {
    prototype: String,
    inline: String
}

pub enum NutModifier {
    Private,
    /// 可由派生类型访问
    /// 在 Java 中是 `protected`
    Family,
    /// 可由一个程序集中的任何代码访问
    /// 如 `pub(crate)`
    /// 在某些语言中使用 internal
    Assembly,
    Public
}

/// location of source code
pub struct NutPosition {
    pub start_line: i8,
    pub start_position: i8,
    pub end_line: i8,
    pub end_position: i8
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
