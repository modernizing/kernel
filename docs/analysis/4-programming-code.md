---
keywords: 
title: "让代码修改代码"
description: 
slug: 
first_name: 
last_name: 
email: 
created_date: 2022-02-12 20:57:46
updated_date: 2022-02-13 16:23:29
---

程序员嘛，重复的事情都应该尽可能自动化。所以，在我们呈现完问题，就要一一去解决问题。

以机器的角度来考虑，对于重构来说，就是发现 bad smell 的模式，寻找解决方案，编程以自动化重构。诸如于 Intellij IDEA 这类的 IDE，以及各类 Lint  工具，便也是类似于此。不过，在已经有了大量的现有工具的情况下，我们编写的工具能做点什么？

* 规模化修改。比起一个个在 IDE 中敲入 `Alt` + `Enter` 来得更有效率 —— 对于大型的工程来说。
* IDE 难以完成的工作。跨多个工程的代码重构，一来是性能问题，二来是不支持。
* 其它不常见的 bad smell 模式

好的习惯不容易学习，但是不好的、便利的习惯，往往非常容易上手。先来看一个简单的 CSS 重构案例。

## 前端：自动化的颜色重构

在诸多的前端项目中，在早期如果没有构建好项目模板，又或者是后期没有按规范捃，那么项目中的颜色中就会分散在各个 CSS 和各类 CSS 预处理器。这个时候，当我们来一个主题类的需求，比如过年的大红色。那么，就需要一个个的 debug。因此，一个比较简单的方式，就是识别代码中的 CSS 中的颜色，提取出来，统一管理。于是，在 2020 年的时候，我和刘宇构建了一个简单的 CSS 重构工具：[Lemonj](https://github.com/modernizing/lemonj)。

思路上也颇为简单：


1. 识别代码中的各类颜色。记录每一个颜色的文件信息，位置信息等。
2. 生成颜色的 mapping 文件。
3. 修改生成的 mapping 文件。通过记录的信息，将颜色值，修改成对应的变量
4. 执行重构。将颜色变量修改到文件中。

从技术实现上，就是使用 Antlr 构建一个跨 CSS 预处理器的颜色解析，主要是针对于 LESS。其中，比较麻烦的一个点在于 CSS 里的颜色，除了 `color` 属性，在 `box-shadow`、`border` 等一系列的属性中都会出现：

```javascript
    switch (propertyKey) {
      case 'color':
      case 'background-color':
      case 'border-color':
      case 'background':
        ...
      case 'border':
      case 'border-right':
      case 'border-left':
      case 'border-bottom':
      case 'border-top':
      case 'border-right-color':
      case 'border-left-color':
      case 'border-bottom-color':
      case 'border-top-color':
      case 'box-shadow':
      case '-webkit-box-shadow':
      case '-moz-box-shadow':
         ...
   }
```

主要分析代码见：[RefactorAnalysisListener.ts](https://github.com/modernizing/lemonj/blob/main/src/RefactorAnalysisListener.ts)。随后，生成一个 Mapping 文件：

```javascript
// _fixtures/less/color/border.less
@color1: #ddd;
// _fixtures/less/color/border.less
@color2: green;
// _fixtures/less/color/rgba.less
@color3: rgba(255, 0, 0, 0.3);
// _fixtures/less/color/sample.less
```

其中的注释信息主要是用于人为的 debug。当然，它还不是全自动化的，后续还需要一系列小的代码修改。但是，大体上已经大大减少了工作量了。随后，我们在这基础上构建了一个简单的 CSS 的 bad smell 识别，用于证明 Antlr 语法的可用性。如下是一个 bad smell 示例：

```javascript
Code Smell:  {
  colors: 24,
  importants: 4,
  issues: 8,
  mediaQueries: 1,
  absolute: 0,
  oddWidth: 1
}
```

这个项目还有一系列的 Todo 要做，有兴趣的小伙伴可以基于此来构建自己的 CSS 重构工具，又或者是接手、完善 [Lemonj](https://github.com/modernizing/lemonj)。

模式上依旧是：识别 bad smell 模式，寻找解析方案，编写自动化重构代码。

## 后端：批量性 Java 代码重构

回到先前说到的 Coca 支持的 Java 代码重构上。同样的，也是识别代码味道的模式，然后重构。场景上是：客户有一个 `common` 的 `common` 包，简单来说，就是上百人的团队，最后维护出一个非常大的 `common` 包，JVM 启动慢 blabla。有些团队离开了这个包，有些团队还在使用，所以需要分析哪些不被使用了。于是，基于 Coca 的分析功能，我们开始构建的第一个例子里，删除未使用的 `import` —— 客户写的代码太烂了。历史有点悠久，当时似乎好像是在 IDEA 中，只要 `import` 的，但是未使用的，也会被视为依赖？。另外一个原因是，代码量较大，一个个过滤成本高。

在有了 AST 的基础上，分析代码就非常简单了：

```go
func BuildErrorLines(node models2.JFullIdentifier) []int {
	var fields = node.GetFields()
	var imports = node.GetImports()

	var errorLines []int
	for index := range imports {
		imp := imports[index]
		ss := strings.Split(imp.Name, ".")
		lastField := ss[len(ss)-1]

		var isOk = false
		for _, field := range fields {
			if field.Name == lastField || lastField == "*" {
				isOk = true
			}
		}

		if !isOk {
			errorLines = append(errorLines, imp.StartLine)
		}
	}

	return errorLines
}
```

从上述代码，其实有一个雷那就是  `lastField == "*"` 此坑嘛，没有填上。然后，就是重构 —— 随机删除代码了：

```javascript
func (j *RemoveUnusedImportApp) Refactoring(resultNodes []models2.JFullIdentifier) {
	for _, node := range resultNodes {
		if node.Name != "" {
			errorLines := BuildErrorLines(node)
			removeImportByLines(currentFile, errorLines)
		}
	}
}
```

只要编译通过了，就说明我们的重构是好的。第一次写 Goland 写了 Coca，所以代码写得比较一般了，不过测试覆盖率有 90%，也算是方便大家对这个代码库重构了。 ![](/processor/blog/images?file_name=2022-02-13T07:05:08.298Z.png)

后续，我用这个项目来向客户证明，嘿，我们的代码都是有测试的，你不需要 100%，只需要 90% 即可（手动狗头）。

Coca 还有比较简单的批量移动 + 重命名功能。速度比 IDEA 高效 + 快速，至少放在当时，客户的机器 + 他们的代码量，IDEA 就未响应了。通过如下的配置形式，以支持重命名 + 移动：

```javascript
move.a.ImportForB -> move.b.ImportForB
move.c.ImportForB -> move.d.ImportForB
```

简单易懂，还相当的靠谱（我觉得），下班回去后一两个小时就能写完 —— billable 时间写不了。

当然，在 IDEA 支撑得住，代码量小的情况下，还是告诉客户你们手动移动吧，然后自己回去想想怎么自动化。

## 让重构消失：构建前置的架构守护

重构，从理论上来说，是一种事后补救的方式。我们应该尽量避免 bad smell 的出现，从 CI 上的 Sonarqube，到 Git Hooks 的 pre check，再到 IDE 里的 Checkstyle，我们无一不是在构建**架构适应度函数**，以让系统的架构逐步演进到合适的状态。

在我们有了代码模型，又有了语法分析能力之后，我们就能构建出一个跨越任何语言的**架构守护工具**，类似于 [ArchUnit](https://github.com/TNG/ArchUnitNET)。好的架构模式、设计模式，只有变成代码，可测试、可度量，它才有发挥的空间。通过前面的一系列 Antlr 的语法分析基础，很容易就能具备编写一套新的 DSL，再配上老马的《领域特定语言》作为指导思想，《ANTLR 4权威指南》作为实践手册，我们就是一个“代码专家”。

于是呢，我按照这个想法，开了一个坑：[Guarding](https://github.com/modernizing/guarding)  一个用于 Java、JavaScript、Rust、Golang 等语言的架构守护工具。结合 TreeSitter 进行目标代码的模型构建，借助于易于理解的 DSL，来编写守护规则。在设计上参考了 ArchUnit 的语法，采用了 Rust 里的 pest 作为解析器 —— 主要是一年前 Rust 的 Antlr 支持不好（完整的语法：[guarding.pest](https://github.com/modernizing/guarding/blob/master/guarding_parser/src/guarding.pest)）：

```javascript
normal_rule = {
	rule_level ~ ("(" ~ scope ~ ")")? ~ (use_symbol ~ expression)? ~ should? ~ only? ~ operator ~ assert ~ ";"?
}

rule_level = {
    "package" |
    "class" |
    "struct" |
    "function" |
    "file"
}

use_symbol = {
    "::" |
    "->"
}
```

rule_level 对应 ArchUnit 里的 CodeUnits，后面的 `operator` 和 `assert`便是核心的计算逻辑所在。最后的规则示例：

```javascript

// class
class(implementation "BaseParser")::name should endsWith "Parser";
class("java.util.Map") only accessed(["com.phodal.pepper.refactor.staticclass"]);
class(implementation "BaseParser")::name should not contains "Lexer";

// naming
class("..myapp..")::function.name should contains("Model");

// 简单的值计算
package(".")::file.len should < 200;
package(".")::file.len should > 50;
```

代码中的 `::` 可以换成 `→` 表示，都是在 `use_symbol` 中声明的，自己写的语法嘛，怎么开心就这么写。最后，代码是可以 work 的，也没有枉费我看了许久的 ArchUnit 源码。

顺带一提，先前提到的 TreeSitter 的 S 表达式还挺好玩的，有空应该实现一个：

```javascript
(using_directive
	(qualified_name) @import-name)
```

上述的代码可以用于识别 C# 里的 `using`声明。不过，我在 Guarding 中实现的解析倒是不太好。

## 其它

重构是一件有技巧、有难度的手工活。但是，作为一个工程实践上的专家，我们应该让重构消失。

回到开始，成为一个代码方面的专家非常有意思。






