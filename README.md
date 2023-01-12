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
 
## Configuration

### Global

Add following configuration to `${LAPCE_INSTALL_DIR}/plugins/oknozor.lapce-java/volt.toml`:

```toml

[config."volt.serverArgs"]
default = "--jvm-arg=-javaagent:lombok.jar"  # Example: Lombok Agent
description = "Language Server's Arguments"

[config."volt.serverPath"]
default = "/usr/local/bin/jdtls" # Example: Eclipse JDT Language Server
description = "Path of `Java Language Server` executable. When empty, it points to the bundled binary `jdtls`."

```

### Workspace

Add following configuration to `${WORKSPACE_DIR}/.lapce/settings.toml`:

```toml

[lapce-java]
lombok = false
description = "Language Server's Arguments"

[lapce-java.volt]
serverPath = "/usr/local/bin/jdtls"             # Example: Eclipse JDT Language Server
serverArgs = "--jvm-arg=-javaagent:lombok.jar"  # Example: Lombok Agent

```


 
## Licence

All the code in this repository is released under the Apache License, for more information take a look at
the [LICENSE](LICENSE) file.