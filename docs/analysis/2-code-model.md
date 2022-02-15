---
keywords: 
title: "为代码再建个代码模型"
description: 
slug: 
first_name: 
last_name: 
email: 
created_date: 2022-02-09 19:11:10
updated_date: 2022-02-10 16:00:27
---

为代码建模并不是一件很难的事情，毕竟每个编译器都在重复做同样的事情。

## 从代码到模型

现在，回忆一下你大学学的编译原理 —— 虽然有些人可能和我一样没上过对应的课。

```java
class GFG {
    public static void main(String[] args)
    {
        System.out.println("Hello World!");
    }
}
```

解析成 AST 后，可以用如下的形式来表示（可能没有对照 JVM 里的实现）：

```
CLASS_DEF -> CLASS_DEF [1:0]
|--MODIFIERS -> MODIFIERS [1:0]
|   `--LITERAL_PUBLIC -> public [1:0]
|--LITERAL_CLASS -> class [1:7]
|--IDENT -> GFG [1:13]
`--OBJBLOCK -> OBJBLOCK [1:17]
    |--LCURLY -> { [1:17]
    |--METHOD_DEF -> METHOD_DEF [2:4]
    |   |--MODIFIERS -> MODIFIERS [2:4]
    |   |   |--LITERAL_PUBLIC -> public [2:4]
    |   |   `--LITERAL_STATIC -> static [2:11]
    |   |--TYPE -> TYPE [2:18]
    |   |   `--LITERAL_VOID -> void [2:18]
    |   |--IDENT -> main [2:23]
    |   |--LPAREN -> ( [2:27]
    |   |--PARAMETERS -> PARAMETERS [2:34]
    |   |   `--PARAMETER_DEF -> PARAMETER_DEF [2:34]
    |   |       |--MODIFIERS -> MODIFIERS [2:34]
    |   |       |--TYPE -> TYPE [2:34]
    |   |       |   `--ARRAY_DECLARATOR -> [ [2:34]
    |   |       |       |--IDENT -> String [2:28]
    |   |       |       `--RBRACK -> ] [2:35]
    |   |       `--IDENT -> args [2:37]
    |   |--RPAREN -> ) [2:41]
    |   `--SLIST -> { [2:43]
    |       |--EXPR -> EXPR [3:26]
    |       |   `--METHOD_CALL -> ( [3:26]
    |       |       |--DOT -> . [3:18]
    |       |       |   |--DOT -> . [3:14]
    |       |       |   |   |--IDENT -> System [3:8]
    |       |       |   |   `--IDENT -> out [3:15]
    |       |       |   `--IDENT -> println [3:19]
    |       |       |--ELIST -> ELIST [3:27]
    |       |       |   `--EXPR -> EXPR [3:27]
    |       |       |       `--STRING_LITERAL -> "Hello World!" [3:27]
    |       |       `--RPAREN -> ) [3:41]
    |       |--SEMI -> ; [3:42]
    |       `--RCURLY -> } [4:4]
    `--RCURLY -> } [5:0]
```

对于代码分析来说，我们就是：

1. 构建**类** AST 模型。通过代码分析工具，得到一个类似上述内容的结果，不同的工具得到的详尽程度不同。
2. 基于标准的 AST 构建分析模型。如我们只取类、函数的信息，就需要解析 `CLASS_DEF` 里的 `IDENT` ，以及其 children 中的 `METHOD_DEF` 里的 `IDENT`，遍历-取值，就这么简单。

所以，要构建出一个完善的 AST 及其模型，基本上就是写一个语言的编译器前端。在现代的编程语言里，Rust 能提供一个非常不错的参考，如 Rust 的编译过程是 AST → HIR → MIR → LIR，其官方在引入  MIR 的时候写了一篇博客《[Introducing MIR](https://blog.rust-lang.org/2016/04/19/MIR.html)》

![Rust Flow](https://blog.rust-lang.org/images/2016-04-MIR/flow.svg)

在 Rust 编译器里， HIR 相当于是 Rust 的 AST，它在源码的基础上进行解析、宏扩展和名称解析之后生成。如下是 Rust 的 hello, world! 生成的 HIR 表示：

```rust
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
        {
                ::std::io::_print(::core::fmt::Arguments::new_v1(&["hello, world!\n"],
                        &[]));
            };
    }
```

基于不同阶段的编构建模型，得到的模型结果是不完全一样的。如果我们想分析编程语言调用系统的库，或者是三方的库，那么从这里得到的才是更精确的 —— 对比于 Java 的 Bytecode。Java 编程过程中对于 Annotation 的处理，也在侧面反应两者的差异之处。所以，想获取准备的代码模型，可以在这的基础上，进一步探索下编程语言的构建。

如此一来，你也就是一个真正的代码专家了。

## 0.1 初始化版本：面向 Java

在和新哥设计第一个 MVP 版本的时候，只是想比 Doxygen/Tequila 更准确地记录 Java 代码的调用链。所以，设计的模型也相当的简单：

```go
type JClassNode struct {
	Package     string
	Class       string
	Type        string
	Path        string
	Fields      []JAppField
	Methods     []JMethod
	MethodCalls []JMethodCall
}
```

如上的历史代码所示，在面向 Java 语言设计，只记录一个类（Class）的名称、包名、类型、路径、成员变量（包含了依赖的类型）、函数/方法、函数调用关系。因为最初只是为依赖设计，所以调用关系只保存在 ClassNode 里。基于 Antlr 这样的解析器生成器之后，其对应的解析代码（Listener 模式）也颇为简单（[java_full_listener.go](https://github.com/modernizing/coca/blob/master/pkg/infrastructure/ast/ast_java/java_full_listener.go)）：

```go
func (s *JavaCallListener) EnterClassDeclaration(ctx *ClassDeclarationContext) {
	currentType = "Class"
	currentClz = ctx.IDENTIFIER().GetText()

	if ctx.EXTENDS() != nil {
		currentClzExtends = ctx.TypeType().GetText()
	}
}
```

从 ClassDef/ClassDecl 中获取  ident 就是类名，如果有 extends 关系的话，再获取 extends 关系。这是一个初始化的版本，所以没有考虑到非常复杂的场景，比如多重继承、泛型等等。

但是，它也让我重新理解了一下，为什么有的语言的语法设计得有点诡异 —— 解析器不好写。

## 1.0 版本：更多的工具，更多的模型

在发布了 Coca 之后，从 GitHub 几百的 star 和对应的迁移指南 [Migration](https://github.com/phodal/migration) 2.8k 的 star 来看，这个领域的需求还是相当的旺盛。

所以，我们开发了更多的功能，也一步步陷入了「人类创造的三个系统」的陷阱中。

### 适用于重构的模型

而后，为了生成实现不适用在 IDE 用的重构功能（多代码库引用检测、类移动等），我们又构建了一个新的模型，因为我们就只需要这么多信息：

```go
type JFullMethod struct {
	Name              string
	StartLine         int
	StartLinePosition int
	StopLine          int
	StopLinePosition  int
}

type JField struct {
	Name   string
	Source string
	StartLine         int
	StopLine          int
}

type JPkgInfo struct {
	Name   string
	StartLine         int
	StopLine          int
}
```

这个时候要实现的功能，还是比较简单的，所以并不是那么复杂

### 适用于测试代码坏味道的模型

除了重构之后，在 Coca 中，还有一个非常有意思的特性：测试代码坏味道。测试代码坏味道，是指单元测试代码中的不良编程实践（例如，测试用例的组织方式，实现方式以及彼此之间的交互方式），它们表明测试源代码中潜在的设计问题。简单来说，就是看测试是否有断言？ignore 的测试数量等等。需求不复杂，所以构建的模型也比较简单：

```go
type BSDataStruct struct {
	core_domain.CodeDataStruct

	Functions    []BSFunction
	DataStructBS ClassBadSmellInfo
}

type BSFunction struct {
	core_domain.CodeFunction

	FunctionBody string
	FunctionBS   FunctionBSInfo
}
```

当然了，细节都是魔鬼，比如 `FunctionBSInfo` 长什么样的？

## 2.0 AST 集合：一个臃肿而缓慢的系统

随后，我们试图构建一个更理想的系统，于是就有了「第二个系统」，一个经过精心设计的系统。

### 兼容更多的语言

随着 Coca/Chapi 的演进，陆陆续续想支持 Golang、Java、Python 等语言。于是，一个平凡的 ClassNode 已经变成了 DataStruct：

```kotlin
@Serializable
open class CodeDataStruct(
    var NodeName: String = "",
    var Type: DataStructType = DataStructType.EMPTY,
    var Package: String = "",
    var FilePath: String = "",
    var Fields: Array<CodeField> = arrayOf(),
    var MultipleExtend: Array<String> = arrayOf(),
    var Implements: Array<String> = arrayOf(),
    var Extend: String = "",
    var Functions: Array<CodeFunction> = arrayOf(),
    var InnerStructures: Array<CodeDataStruct> = arrayOf(),
    var Annotations: Array<CodeAnnotation> = arrayOf(),
    var FunctionCalls: Array<CodeCall> = arrayOf(),
    @Deprecated(message = "looking for constructor method for SCALA")
    var Parameters: Array<CodeProperty> = arrayOf(), // for Scala
    var Imports: Array<CodeImport> = arrayOf<CodeImport>(),    // todo: select node useonly imports
    var Extension: JsonElement = JsonObject(HashMap())
) {  
    ...
}
```

一味地进行了兼容设计，导致它变得异常复杂。而和多数系统一样，这种兼容设计并非是最理想的，没有进一步做一些抽象，比如函数的属性，参数、返回类型等，是否能构建 [Type Signature](https://en.wikipedia.org/wiki/Type_signature)？虽然这是一个技术项目，但是也掉入了同样的业务模型的常见问题中。

最后，我尝试将非 Java 语言分离成插件，但是因为 Golang 当时的版本并不支持插件化架构。所以，从形态拆分为了 Java + 其它语言 CLI，并转向了 Rust 语言。

### 更多的模型

既然，原来的模型可能太重了，那么是不是会有新模型。所以，陆陆续续又构建了一系列的模型。如，在设计 Guarding/Modeling 的时候，我们也构建了一个简化的版本：

```rust
#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeClass {
    pub name: String,
    pub package: String,
    pub extends: Vec<String>,
    pub implements: Vec<String>,
    pub constant: Vec<ClassConstant>,
    pub functions: Vec<CodeFunction>,
    pub start: CodePoint,
    pub end: CodePoint
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeFunction {
    pub name: String,
    // todo: thinking in modifier
    pub vars: Vec<String>,
    pub start: CodePoint,
    pub end: CodePoint
}
```

在这个版本里，使用的是 TreeSitter 没有函数调用，所以显得非常简单 —— 只是记录基本的类、函数信息等，又是一个非常简单的初始化版本。

## 3.0 进行中的向下抽象：MIR <=> AST

既然，从 AST 做合集太复杂，那么是不是往下钻，寻找更 common 的元素，就能获得更通用的结果，毕竟最后运行在机器上的是一样的。

所以，这就成为了 2022 年的一个潜在的业余乐趣，如果你有兴趣，欢迎到 GitHub 上讨论：[https://github.com/modernizing/kernel](https://github.com/modernizing/kernel)

### 潜在的路：MIR

在这一方面，Rust 编译器的 MIR 就是一个不错的参考，它基于控制流图、也没有嵌套表达式，并且所有类型都是完全显式的 —— 更多的细节可以查看官方的文档：[Rust MIR](https://rustc-dev-guide.rust-lang.org/mir/index.html)。除此，你可以在 <https://play.rust-lang.org/>  里，查看 Rust 在 HIR、MIR、LLVM IR 不同阶段的形式，当然直接诗源码是最简单的。如下是一段 Rust 的代码（本来应该用 Hello, world!，但是它更复杂）。

```rust
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
}
```

生成的 CFG 示例

```javascript
    ...
    bb0: {
        _1 = Vec::<i32>::new() -> bb1;
    }

    bb1: {
        _3 = &mut _1; 
        _2 = Vec::<i32>::push(move _3, const 1_i32) -> [return: bb2, unwind: bb5];
    }

    bb2: {
        _5 = &mut _1;                    
        _4 = Vec::<i32>::push(move _5, const 2_i32) -> [return: bb3, unwind: bb5];
    }
    ...
```

在这个阶段，MIR 比 AST 添加了更完整的细节 —— 我们能知道 `push` 方法是从哪里来的，不需要自己做一些推断。

与之相类似的，还有一个名为 [MIR Project](https://github.com/vnmakarov/mir) 的项目更有意思，它尝试建立多语言的抽象。只是从形式上来看，它接近于 LIR：

```c
hello_m:  module
          import printf
hello:    func i64
          local i64:r # local variable has to be i64
# prototype of printf
p_printf: proto i32, p:fmt
format:   string "hello, world\n"
          call p_printf, printf, r, format
          ret r
          endfunc
          endmodule
```

不过，在代码模型上，还是接近于 MIR 的：

```c
/* Function definition */
typedef struct MIR_func {
  const char *name;
  MIR_item_t func_item;
  size_t original_vars_num;
  DLIST (MIR_insn_t) insns, original_insns;
  uint32_t nres, nargs, last_temp_num, n_inlines;
  MIR_type_t *res_types;
  char vararg_p;           /* flag of variable number of arguments */
  char expr_p;             /* flag of that the func can be used as a linker expression */
  VARR (MIR_var_t) * vars; /* args and locals but temps */
  void *machine_code;      /* address of generated machine code or NULL */
  void *call_addr;         /* address to call the function, it can be the same as machine_code */
  void *internal;          /* internal data structure */
} * MIR_func_t;
```

它让我重新思考起，我如何去组件 Struct/Class 和 Function 的关系？从 AST 的层面来说，这个不好解决，但是从 MIR/LIR 的话，这个问题就变得异常简单了 —— 在底层没有继承。

所以，我们应该如何去设计这样一个模型呢？

### 还有 CLR 和 Graal IR ?

在先前设计 Chapi 的期间，鹏飞推荐了一本书《CLR via C#》，在设计 Chapi 的时候，参考了一部分。简单来说，就是 Microsoft .NET Framework 里构建了一个**公共语言运行时（Common Language Runtime，CLR）**。其核心功能（如内存管理、程序集加载、安全性、异常处理和线程同步）可面向 CLR 的所有语言使用。我并不关心 CLR 怎么实现，我关心的是其中的 “通用类型系统”（Common Type System）。

另外一个有意思的项目就是 Graal VM，它是一个生态系统和共享运行时，不仅提供基于 JVM 的语言（如Java，Scala，Groovy和Kotlin）的性能优势，还提供其他编程语言（如JavaScript，Ruby，Python和R）的性能优势。其中的 Graal IR 便是 Graal 的核心构建块之一：[GraalVM Compiler](https://github.com/oracle/graal/tree/master/compiler)，这个可以作为下一个阶段的研究的乐趣。

这部分足够让我们重新思考一下：公共语言模型是怎样的？

## 其它

我并非编译器方面的专家，更细节的内容可以自己去读代码或者比编译原理相关的书籍。除了传统的龙书、虎书、鲸书，在编译器前端上，Antlr 作者编写的《编程语言实现模型》和后续的《ANTLR 4权威指南》能更快速地帮你入门语法解析。

其它常见问题：

* 没有类型怎么办？诸如于 JavaScript 这一类动态语言，就需要自己尝试性地做一些类型推断。
* 在底层 MIR 真的能做融合吗？不确定，但是可以试试，毕竟有上述的 MIR 大佬说可以：[mir](https://github.com/vnmakarov/mir)。

欢迎来构建和设计新的代码模型：[https://github.com/modernizing/kernel](https://github.com/modernizing/kernel)，然后重构已有的工具。
