---
keywords: 
title: "modernizing：代码可视化代码"
description: 
slug: 
first_name: 
last_name: 
email: 
created_date: 2022-02-10 15:00:46
updated_date: 2022-02-12 15:43:34
---

有了模型，有了分析代码，便可以将代码序列化为一个个的数据。接着，剩下的就是对数据进行操作的过程了。

根据不同的可视化需求，我使用过一系列的可视化形式，如下表：

| 工具 | 编码成本 | 可定制性 | 代码可维护性 | UI 美观性 | 额外工具 | 主要使用场景 |
|----|----|----|----|----|----|----|
| Graphviz | 低（`→`形式的DSL） | 差 | 高 | 低 | 是（打包到工具中，需要考虑跨平台） | 快速构建 PoC，诸如于依赖可视化 |
| PlantUML | 低（标准化的格式） | 差 | 高 | 低 | 是 | 面向模型建模 + IDE的可视化支持 |
| Web 之 D3.js | 高 | 高 | 低 | 取决于设计 | 不需要，可以打包到工具中 | 可交互性要求高，如依赖、调用分析 |
| Web 之 Echats/AntV | 低 | 中 | 中 | 高 | 不需要，可以打包到工具中 | 可交互性要求高，如依赖、调用分析 |
| Web 3D 之 Three.js | 高 | 高 | 低 | 取决于设计 | 不需要，可以打包到工具中 | 3D 交互、 VR 世界 |

总的来说，我们依旧是在各种的平衡，每个工具都有自身的特点和优势。在代码模型不一致的时候，我们需要一层 adapter，用于从代码模型转换到可视化所需要的模型，进而将代码可视化。在这个时候，就能体现出会写前端代码的好处。从模式上来说，主要是分为两类：


1. 利用已有工具的静态可视化。比较常见的有，诸如于使用 Dot 语言描述的 Graphviz，使用 UML 描述的 PlantUML。
2. 开发新工具的交互性可视化。常见的有 Web 技术开发的工具，如 D3.js 等。

相似的，和代码分析一样，也需要一个成本的考虑。从无到有，优先考虑已有的工具；从 1 到 100，便是考虑自己做个可视化工具。

## 静态的代码可视化

这里主要以我使用过的 Graphviz、PlantUML 作为示例。

### 神器 Graphviz：依赖可视化

Graphviz 是自 1991 年开发的，历史悠久，比较从使用频率来看，它应该是用得最多的一类工具。参见 [Graphviz 的 wiki](https://en.wikipedia.org/wiki/Graphviz)，诸如于 Doxygen、Rust、Sphinx 等大量的工具都会用它来生成文档中的图形，而像 OmniGraffle 这一类工具，则使用它来生成自动化布局。从场景上来看，主要就是利用它便利的 Dot 语言描述，结合图形算法，来自动生成依赖关系。

Graphviz 中的 Dot 语言非常便利，只需要使用 `→` 这样的语法，就可以生成调用关系。如下是 Coca 中生成调用链的 dot 文件示例：

```javascript
digraph G { 
  "POST /books" -> "com.phodal.pholedge.book.BookController.createBook";
  "com.phodal.pholedge.book.BookController.createBook" -> "com.phodal.pholedge.book.BookService.createBook";
  ...
}
```

对应转换后的图形如下所示（因为是测试代码中有多个相同的 Controller，所以是双份箭头）：

 ![](/processor/blog/images?file_name=2022-02-12T03:56:27.101Z.png)

对于代码量较大的工程来说，生成的 SVG 就会比较大，以致于可能会在浏览器上渲染许久。为此，常见的一种解决方案就是：添加大量的 filter 函数、参数，以有选择性的过滤。这也造成了另外一个问题，工具的学习成本和试命令的成本比较高。有一个很好的例子就是，虽然我是 Coca 的作者，但是很多功能，我现在已经不记得了。

### PlantUML：模型可视化

和 Graphviz 相比，UML 更为人所知，是个建模的好工具。**PlantUML** 是一个开源工具，能让你通过纯文本的方式来生成 UML 图（Unified Model Language 统一建模语言）。在 Modernizing 的几个工具里，主要是用它来对模型进行可视化，诸如于：

* [Modeling](https://github.com/inherd/modeling)，结合 Ctags 对代码库中的模型（如 repository）进行分析，结合 id 等，构建出简单的依赖关系。
* SQLing，结合 MySQL parser 对数据库的 Schema 进行分析，结合外键关系，构建出表的依赖关系，进而帮助我们推导出模型的关系。

以 SQLing 为例，如下是一个网上找的 SQL 代码：

```sql

CREATE TABLE human(
    ...
)
CREATE TABLE car(
    id VARCHAR(12) PRIMARY KEY,
    mark VARCHAR(24),
    price NUMERIC(6,2),
    hid VARCHAR(12),
    CONSTRAINT fk_human FOREIGN KEY(hid) REFERENCES human(id)
)
```

通过 SQLing，可以转换为如下的结果（UML）：

```javascript
@startuml
class Human {
 ...
}
class Car {
 - Id: String
 - Mark: String
 - Price: BigDecimal
 - Hid: String
}
Car --> Human
@enduml
```

这样一来，就可以配合 IDEA 的 PlantUML 插件进行可视化了：

 ![](/processor/blog/images?file_name=2022-02-12T01:48:59.454Z.png)

Modeling 的依赖构建会比 SQLing 复杂一些，在构建模型的时候，还要从 `xxId` 中尝试分析出是否存在这样的类，以构建出对应的依赖关系 —— 当然，这种是基于编码模式的分析，有些人的代码写的是 `id` 没有前缀，这就分析不出来了。

## 交互的代码可视化

在基于微服务、代码库小的场景下，上述的 Graphviz、PlantUML 基本上可以完成大部分的工作。而对于遗留系统来说，它巨大的代码量，就意味着我们需要更强的交互工具。所以，我找了个周末写了个工具：[Merry](https://github.com/modernizing/merry)。

### 从 Graphviz 到 D3.js：OSGi 的天坑

我尝试构建的第一个场景是一个 OSGi 系统的 Ant 转移到 Maven 方案上，我们的目标是告诉客户：你还不如重写。不过，你需要有强壮的证据，还有可估算的成本证明。采用 OSGi 框架，就意味着系统可能有几十、几百个 bundle，可以理解为模块，而这些模块又可以相互依赖，妥妥的一个大泥球。与此同时，采用 Ant 又意味着系统的依赖是放在某个目录里管理的，具体的版本什么的，也不定会在文件名中体现。所以，我们所要做的就是：


1. 解析 `build.xml`，从中获取 `classpath` 中的 jar 路径。
2. 解析 jar 包中的 Manifest.MF、pom.properties，从中解析出包名、版本号、Export、Import 等一系列的信息。
3. 自动生成一个 pom.xml 文件。（PS：需要对一些依赖进行人工校验，所以是半自动的。可以通过配置 map 文件，在后续变成全自动化。）

其中，最过于坑人的，要数 Manifest.MF 存在多个不同的版本的问题。在使用正则无力的情况下，最后只能用 Antlr 来写解析器了。有意思的是，OSGi 生成的 Manifest.MF 里，必须有 `Import-Package` 和 `Export-Package`，便可以从中生成项目的依赖信息。就这么找了 Apache 的 OSGi 项目，run 了一下，写了个 demo，it works：

 ![](/processor/blog/images?file_name=2022-02-12T06:46:17.034Z.png)

然后，来到客户现场，一试，嘿，傻眼了，客户有几百个 bundle。怎么看清包之间的关系，怎么看清哪个 bundle 被依赖最多？所以，让 D3 来干活吧。

### 依赖图

在有了依赖关系之后，只需要生成一个 JSON 文件，就可以给 D3.js 使用了。剩下要做的就是打包 Web 应用，以便于在客户的 Windows 电脑上运行 —— 这就体现出了 Golang 的跨平台优势。在采用有了 GitHub Action 的多平台构建之后，Rust 也可以实现同样的效果。接着，迅速实现了个 demo，然后拿 Eclipse 的 OSGi 框架 Equinox 跑了一下，这图估计也 hold 不住，几百个 bundle：

 ![](/processor/blog/images?file_name=2022-02-12T06:58:02.989Z.png)

于是，又从 D3.js 的 Gallery 里继续拿个图了测试一下：

 ![](/processor/blog/images?file_name=2022-02-12T07:00:04.392Z.png)

效果比上面好一点，但是依旧不理想。然后，我就一如即往的弃坑了 —— 在 OSGi 技术越来越难见到的时代，投精力开发工具，显得非常不值得。和 D3.js 的简单 demo 相比，我们在 ArchGuard 设计的、基于 AntV G6 的可视化来说，它显得更加的好用。

Merry 可视化的最后 demo 见：<https://modernizing.github.io/merry/demo>

### 可交互的变化

上面的可交互性仅限于当前时期，但是历史上的变化有时候往往更重要。于是，在设计效能分析工具 [Coco](https://github.com/inherd/coco) 时，我们做提分析 Git 的提交历史，从中发现历史上的高频变更。如下是 [Nginx](https://inherd.org/cases/nginx/) 的示例，可以播放，然后查看变化：

 ![](/processor/blog/images?file_name=2022-02-12T07:10:44.460Z.png)

对于本身就是增量变更的 Git 来说，分析 Git 的日志，就能得到上面的结果。但是，对于代码来说，要分析模型上的增量变更，还是稍微有一点麻烦。如果有哪个小伙伴有空，可以去构建这样的功能。

## 面向风口的可视化

几年前，在阅读《Your Code as a Crime Scene》一书之后，我便一直想构建一个 Code City，只是我一直看不到有效的使用场景。在设计 Coco 和 Coca 的时候，虽然图形是 2D 的，表现力是有限的，但是多数时候是够用的 —— 受客户开发机的性能影响。所以，去年在元世界又开始火了之后，结合了几年前在 TW 国内构建的第一个 VR 机器人，并写了 Code City 的 demo：<https://github.com/modernizing/codecity>。

 ![](/processor/blog/images?file_name=2022-02-12T07:17:04.491Z.png)

当然，这还只是一个玩具。只要一打开 Oculus Quest 2，我就沉迷在 Beat Saber 中*。*But the way，我构建了我一直想构建的 Code City demo。

开发工具就是这样的，在业余的时候，需要先搭建个架子，等到使用的时候，就可以改吧改吧上线了。而不是用的时候，发现没有架子，然后就不做了。

### 其它

欢迎入坑，开发更好的 Code City 2.0。

除此，在我们有了 BeeArt 之后， 我们所能做的事情就更多了。可以用 BeeArt 生成的图就可以变为代码模型，又或者是代码模型就可以转到 BeeArt 这样的工具中。当然了，这取决于是不是有开放接口 和 SDK。


