# WhiteHatBot
Discord bot for the WhiteHat Hacking discord server. 

The bot is a Rustlang copy of the javascript based whitehatmail discord bot, made by `[Mod]Basically` on the WhiteHat Hacking discord server.
Credits goes to him for the function ideas and javascript implementation! 

---
## Commands:
There are `3` categories for currently available commands:
1. [Common](#common-commands)
2. [Application](#application-commands)
3. [Owner Only](#owner-only-commands)

---
### Common commands:
| Command        | Description                                                                               |
|----------------|-------------------------------------------------------------------------------------------|
| `multiply a b` | Multiply two numbers (`a * b`)                                                            |
| `divide a b`   | Divide two numbers (`a / b`)                                                              |
| `hacksplain`   | Explain 1337 H4X1NG.                                                                      |
| `howtohack`    | Returns some curated resources in different fields in relation with hacking, programming. |
| `google terms` | Returns an LMGTFY link with the given search terms (`terms`).                             |
| `ping`         | Pong!                                                                                     |
| `hax`          | Learn networking before trying to hack Google                                             |
| `dunning`      | You don't know as much as you think you do.                                               |
| `howtoask`     | Addresses the popular "How to ask a question".                                            |

### Application commands:
| Command | Description                                    |
|---------|------------------------------------------------|
| `apply` | Get information about role application process |

### Owner only commands:
| Command | Description        |
|---------|--------------------|
| `quit`  | Shuts down the bot |
---

## Compilation and running:
If you want to run the bot on a Raspberry Pi 2/3/4, use `rust cross` to compile the executable.
Command:
```cross build +nightly --target aarch64-unknown-linux-gnu --release```
---
