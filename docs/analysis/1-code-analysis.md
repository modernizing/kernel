---
keywords: 
title: "代码语法分析的多种形式"
description: 
slug: 
first_name: 
last_name: 
email: 
created_date: 2022-02-07 15:06:28
updated_date: 2022-02-14 21:41:44
---

代码分析是我们编写自动化重构、架构守护等一系列工具的第一步。而代码分析的方式有多种不同的形态，最常见的是基于源码以及基于编译后的字节码（常见于 Java 语言）的静态程序分析。

通常来说，根据我们的目标获取的信息是不同的，如：类/结构体、成员、函数（含参数、返回值、注解）、引用（import）、表达式等。因此，所选的工具也是不同的：

| 目标 | 语法信息级别 | 可选 工具 |
|----|----|----|
| HTTP API | @注解、参数、类、方法 | 语法分析器（语言自带、三方、Antlr） |
| 领域模型 | 类/结构体、成员等 | 根据不同精度，可以考虑 Ctags、TreeSitter等 |
| 包、类依赖关系 | 引用、函数调用等。 | Doxygen、 语法分析器等 |
| 调用链 | 全部信息 | 语法分析器（语言自带、三方、Antlr） |

根据我们的不同需求，我们还需要记录语法的位置信息。比如，同样是 HTTP API 的情况下，我们想获取：

* API URI 列表。只需要解析注解即可。
* API 的输入和输出参数。注解 + 解析函数签名。
* API 输入到数据库。注解 + 解析函数签名 + 调用链。

因此，是不是使用语言自带的语法分析器，生成一个完整的模型就行了，如 Java 使用 [Javaparser](https://github.com/javaparser/javaparser)。事情并不是这么简单，如今是微服务时代，每个服务都可能使用不同语言，一个二三十人的研发团队，可能使用 7\~8 种语言 —— 为每个服务挑选合适的语言，老系统 C#、新系统  Java、大数据 Scala、AI 用 Python 等。除此，为某个语言写一个成本也是颇高的，并且用处可能还不大。

所以 ，在不断平衡之间，我们有了一系列的工具选型。

## 编译器前端

> *编译器*粗略分为词法分析，语法分析，类型检查，中间代码生成，代码优化，目标代码生成，目标代码优化。

### 基于语法分析器（parser）

从实现的层面来看，使用官方的 parser 是最准确的 —— 前提是它提供了便利的接口，像 Java 语言好像就没有这样的接口。

* 官方支持。如 [Coca](https://github.com/modernizing/coca) 早期在解析 Golang 时，使用的是 Go 的 [parser](https://pkg.go.dev/go/parser) 包。
* 三方。在 [SQLing](https://github.com/modernizing/sqling) 中，我们使用的 TiDB 的 [parser](https://github.com/pingcap/parser)，它宣称与 MySQL 完全兼容，并尽可能兼容 MySQL 的语法。

使用这一类 parser 比较麻烦的是在于跨语言的支持，每实现一个新的语言，就需要实现一套，不能复用。

## 自制 parser

为了实现更好的跨平台，以及更好玩，选用一个合适的解析器生成器就更“科学” 了。在这一方面，除了传统的 Flex 和 Bison，Antlr 也是一个不错的选择 —— 多语言支持：JavaScript、Golang、Java、Rust 等。

Antlr 社区维护了一个语法库：<https://github.com/antlr/grammars-v4/>，内置了几十种编程语言的 Antlr 语法文件。虽然，部份语法可能不太准确，需要我们手动进行修改，但是依旧可以大大减少我们的编写成本 —— 除了学习 Antlr 是个成本。Antlr 之类工具的迷人之处在于：你可以重温一下《编译原理》，又或者是《计算机程序的构造和解释（SICP）》，毕竟它是编译器的前端部分。你再掌握一下 LLVM 的 API，就可以开发个语言了。它的挑战之处在于，你需要知道语言的各类语法细节，所以也是一个不错的学习新语言语法的机会。

不过，诸如 Java、C++ 等支持在编译时进行代码生成的语言，也会遇到一系列的挫折。诸如于：

* 引用推断。最难受的 `junit.*`需要做一些推断
* 生成工具推断。如 lombok 等

所以，我们需要通过编译过程中的中间表示，来做一些额外的处理。

### 基于**中间表示（IR）**

**IR**-Intermediate Representation（**中间表示**）是程序编译过程中，源代码与目标代码之间翻译的中介。

为了提升语法分析的精准度，就需要应对编译其的代码生成，于是，就需要分析 IR。如：Java 里的 ASM。能对 `.class` 文件进行分析。只是，IR 处理了一些信息，所以如 class 文件里有些内容（如 annotation）好像并不会被记录行号信息，详见：[LineNumberTable Attribute](https://docs.oracle.com/javase/specs/jvms/se8/html/jvms-4.html#jvms-4.7.12)。

 ![Java Flow](http://openjdk.java.net/groups/compiler/doc/compilation-overview/javac-flow.png)

Java、Android 在编译过程中对于 Annotation 的操作，又或者是在编译后的骚优化，也是 666。

不过，它能完成大部分我们所需要的工作。

## 编辑器语法树

编辑器在做语法高亮的时候，也在做类似的事情。正好，我先前在某 spike 过编辑器 / IDE 的架构和实现。

* Atom/VSCode。主要由 JSON/PList 格式的 TMLanguage（源自 TextMate） + 正则表达式实现，即 [VSCode TextMate](https://github.com/microsoft/vscode-textmate) 和 [Oniguruma](https://github.com/kkos/oniguruma) 共同构成了 VSCode 的一部分语法高亮功能。吐槽一句，非常难以维护。
* Eclipse。需要手写解析器，[FAQ How do I write an editor for my own language?](https://wiki.eclipse.org/FAQ_How_do_I_write_an_editor_for_my_own_language%3F)。
* Intellij IDEA。可以通过  BNF 来添加相应的功能：[Custom Language Support Tutorial﻿](https://plugins.jetbrains.com/docs/intellij/grammar-and-parser.html#define-the-grammar)。
* Vim。由自带的 Vim 脚本 + 正则表达式（类似）来实现，示例：[Rust.vim](https://github.com/rust-lang/rust.vim/blob/master/syntax/rust.vim)
* Emacs。由 Emacs Lisp 语言 + 正则表达式（类似）来实现，示例：[rust-mode](https://github.com/rust-lang/rust-mode/blob/master/rust-mode.el)

只是呢，上述的工具，在离开了编辑器之后，这个 API 嘛，就有些难用了。于是，有一些独立的工具出现了。

### 基于语言服务器（LSP）

虽然，我还没有尝试过使用 LSP 来实现语法分析，但是我尝试构建过一个语言及其 LSP。因此，从理论来说，LSP 也能达成此目的。并且与 Antlr 类似，Microsoft 也维护了一个 LSP 的目录：<https://microsoft.github.io/language-server-protocol/implementors/servers/>。

麻烦的是，不同语言的 LSP 可能由不同的语言来实现，在系统的集成上会比较困难。其所需要的语言运行环境比较多，比如 Java 的就需要一个 JDK/SDK，在编写分析工具时，自动化测试环境搭建起来也比较麻烦。

### Ctags：有限的解析

Ctags 可以快速实现对类、成员的解析，所以它经常被用在 Vim 的语法高亮上。只是呢，使用 Ctags 难以实现支持：某个函数调用了哪些函数、哪些函数被某个函数调用。从流程上，先用 ctags 生成 tags 文件，然后解析这个 tags 文件即可。如下是一个 tags 文件（部分）：

```javascript
MethodInfo	src/coco_struct.rs	/^pub struct MethodInfo {$/;\"	struct	line:21	language:Rust
name	src/coco_struct.rs	/^    pub name: String,$/;\"	field	line:22	language:Rust	struct:MethodInf
```

然后，再写几个正则表达式 match 一下：

```javascript
        Regex::new(r"(?x)/\^([\s]*)
([A-Za-z0-9_.]+)
(,(\s|\t)*([A-Za-z0-9_.]+))*(\s|\t)*
(?P<datatype>[A-Za-z0-9_.<>\[\]]+)").unwrap();
```

因此，在不考虑正则表达式难写和代码精准度的情况下，使用 Ctags 还会存在一些小问题：


1. 版本冲突，如 macOS 环境自带了一个 ctags，需要 override，或者自定义路径。
2. 下载 ctags。特别是如果客户是在内网环境时，又会比较麻烦。

所以，TreeSitter 成了一个更好的选择：平衡。

### TreeSitter

Tree-sitter 是一个解析器生成工具和增量解析库。 它可以为源文件构建具体的语法树，并在编辑源文件时有效地更新语法树。这个工具最初是为 Atom 编辑器设计的。TreeSitter 内置了一个 S  表达式，可以快速构建出我们想要的模型。如下是一个 C# 代码：

```csharp
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Syntax;

[ApiController]
public class SharpingClassVisitor { 

}
```

对应的 S 表达式如下：

```javascript
(using_directive
	(qualified_name) @import-name)

(class_declaration
    (attribute_list (attribute name: (identifier) @annotation.name))?
    name: (identifier) @class-name
)
```

我们在 Guarding 中使用了 TreeSitter 来实现，示例：\[Guarding Ident\](<https://github.com/modernizing/guarding/tree/master/guarding_ident/src/identify>)，与 Ctags 相比，没有这个环境依赖，会比较清爽。

其在线 Background：<https://tree-sitter.github.io/tree-sitter/playground> 。

## 其它生成工具

除了上述的几类，还有一些可选的工具。

### 文档生成器：Doxygen

Doxygen 是一个适用于 C++、C、Java、Objective-C、Python、IDL、Fortran、VHDL、PHP、C# 和 D 语言的文档生成器。为了生成代码的文档，它需要能支持对于代码进行语法分析。所以，它也内置了**有限**的语法分析功能。

在 [Tequila](https://github.com/modernizing/tequila) 中，是通过分析 Doxygen 生成的文档结果，从而构建出内部的依赖关系。如下是一个 Doxygen 生成的 Graphviz 文件：

```javascript
digraph "Domain::AggregateRootB"
{
 // LATEX_PDF_SIZE
  edge [fontname="Helvetica",fontsize="10",labelfontname="Helvetica",labelfontsize="10"];
  node [fontname="Helvetica",fontsize="10",shape=record];
  Node1 [label="Domain::AggregateRootB",height=0.2,width=0.4,color="black", fillcolor="grey75", style="filled", fontcolor="black",tooltip=" "];
  Node2 -> Node1 [dir="back",color="midnightblue",fontsize="10",style="solid",fontname="Helvetica"];
  Node2 [label="Domain::AggregateRoot",height=0.2,width=0.4,color="black", fillcolor="white", style="filled",URL="$class_domain_1_1_aggregate_root.html",tooltip=" "];
  Node3 -> Node2 [dir="back",color="midnightblue",fontsize="10",style="solid",fontname="Helvetica"];
  Node3 [label="Domain::Entity",height=0.2,width=0.4,color="black", fillcolor="white", style="filled",URL="$class_domain_1_1_entity.html",tooltip=" "];
}
```

解析这个 `dot` 文件，从而生成项目的类与类之间的依赖信息。

### 索引工具：CodeQuery

[CodeQuery](https://github.com/ruben2020/codequery) 是由 GitHub 推出的索引和查询工具，它主要结合了 Ctags 和 Cscope，cscope 可以实现部分语言的表达式（expression）的支持。它试图结合 cscope 和 ctags 提供的功能，提供比 cscope 更快的数据库访问（因为它使用 sqlite）。虽然，我还没有试过，但是应该也是可以玩一玩的。架构如下所示：

 ![CodeQuery workflow](https://github.com/ruben2020/codequery/raw/master/doc/workflow.png)

它结合了 starscope、pyscope、cscope 等多个工具，来实现对于代码的解析。

## 结论

更多详细内容，可以阅读 Modernizing 中的相关项目源码，欢迎参与使用和参与到其中 \~。