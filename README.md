Lapce Java
---
[Lapce](https://lapce.dev/) LSP plugin for java, powered by Eclipse JDT Language Server


## Installation

To install from source use the provided [just](https://github.com/casey/just) recipes:
 
**Lapce stable plugin:**
```shell
❯ just install-stable
```
 
**Lapce debug plugin:**
```shell
❯ just install-debug
```
 
## Initializing Projects

If you are using Maven for your Java Project, you have to initialize your project for Eclipse JDT using:
```shell
❯ mvn eclipse:eclipse 
```

Without initializing your project, Eclipse JDT won't work.
 
## Licence

All the code in this repository is released under the Apache License, for more information take a look at
the [LICENSE](LICENSE) file.
