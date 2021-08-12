# WhiteHatBot
Discord bot for the WhiteHat Hacking discord server. 

The bot is a Rustlang copy of the javascript based whitehatmail discord bot, made by `[Mod]Basically` on the WhiteHat Hacking discord server.
Credits goes to him for the function ideas and javascript implementation! 

2 functions available for now
    
    --howtohack
        Returns some curated resources in different fields in relation with hacking, programming.

    --google
        Create a lmgtfy link with the given search terms. Sentences will be urlencoded.


If you want to run the bot on a Raspberry Pi 2/3/4, use `rust cross` to compile the executable. 

cross build +nightly --target aarch64-unknown-linux-gnu --release
