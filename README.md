# WhiteHatBot
Discord bot for the WhiteHat Hacking discord server. 

2 functions available for now
    `--howtohack`
        Returns some curated resources in different fields in relation with hacking, programming.

    `--google`
        Create a lmgtfy link with the given serach terms. Sentences will be urlencoded.


If you want to run the bot on a Raspberry 2/3/4, use `rust cross` to compile the executbale. 

cross build --target aarch64-unknown-linux-gnu
