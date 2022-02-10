pub struct NutProject {
    pub manifest: NutManifest,
}

/// module/package manifest
pub struct NutManifest {
    pub major_version: String,
    pub minor_version: String,
    pub build_number: String,
    pub revision_number: String,
    pub flags: Vec<String>,
    /// static files
    pub resources: Vec<String>,
    pub additions: Vec<String>,
    pub exported_types: Vec<String>,
}

/// todo: add document test
/// >>> test_cases
/// Java: `import java.io.BufferedReader`
/// TypeScript: `import { ZipCodeValidator as ZCV } from "./ZipCodeValidator"`
/// Rust: `use deeply::nested::function as other_function`;
/// <<<
pub struct NutImport {
    pub source: String,
    pub as_name: String,
    pub usage_name: String,
    pub scope: String,
}

/// Module is a high level entity of Nut program
/// equal to `Assembly` in CLR
pub struct NutModule {
    name: String,
    items: Vec<NutItem>,
    export: Vec<String>,
    import: Vec<String>,
}

pub struct NutFunction {
    // signature
    name: String,
    instructions: Vec<NutInstructionCode>,
}

/// Prototype
/// in MIR, use proto to save call method, and parameters
///```mir
///p_printf: proto p:fmt, i32:result
///p_sieve:  proto i32, i32:iter
///```
pub struct NutProto {
    name: String,
    // alias to `type`
    proto_type: NutDataType,
    args: Vec<String>,
}

/// TypeRef in CLR
pub struct NutTypeRef {
    name: String,
    flags: Vec<NutModifier>,
    functions: Vec<NutFunction>,
    fields: Vec<NutItem>,
    properties: Vec<NutItem>,
    event: Vec<NutItem>,
}

pub struct NutStr {
    len: u8,
    string: String,
}

pub struct NutName {
    name: String,
}

/// NutItem
/// key value is `function`, `prototype` and `import`
pub enum NutItem {
    Function(NutFunction),
    Prototype(NutProto),

    /// todo: spike on remove
    ImportID(NutName),
    ExportId(NutName),
    ForwardId(NutName),
    // todo: ref data
    Data(String),
    // todo: ref data
    RefData(String),
    // todo: expr data
    ExprData(String),
}

/// parameter
pub struct NutParam {
    flag: ParamFlag,
}

pub enum ParamFlag {
    In,
    Out,
    Retval,
}

pub struct NutProperty {}

pub enum NutStructModifier {
    Virtual,
    New,
    Override,
    Sealed,
    Abstract,
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
    inline: String,
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
    Public,
}

/// location of source code
pub struct NutPosition {
    pub start_line: i8,
    pub start_position: i8,
    pub end_line: i8,
    pub end_position: i8,
}

pub struct NutInline {}

pub enum NutDataType {
    /// Integer types of different size:
    IntegerType(NutIntegerType),
    /// Float or (long) double type
    FloatType(NutFloatType),
    /// Pointer, memory block
    PointerType(NutPointerType),
    /// type
    TypeType(NutTypeType),
    /// return block
    ReturnBlock(NutReturnBlock),
    /// UNDEF, BOUND ? may be array
    CollectionType(NutCollection),
}

pub enum NutIntegerType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
}

pub enum NutFloatType {
    Float,
    Double,
    LongDouble,
}

pub enum NutPointerType {
    Pointer,
    MemoryBlock,
}

pub enum NutReturnBlock {}

pub enum NutCollection {
    Undef,
    Bound,
}
