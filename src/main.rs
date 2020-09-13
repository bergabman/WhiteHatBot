#![allow(non_snake_case)]
use anyhow::Result;
use serde_derive::Deserialize;
use serenity::model::{channel::Message, gateway::Ready, id::ChannelId};
use serenity::{prelude::*, utils::Colour};
use urlencoding::encode;

#[derive(Debug, Deserialize)]
struct Config {
    marker: String,                 // Marker string to recognise discord messages sent to the bot, defind in botconfig.toml.
    own_bot_token: String,          // Own bot token used to connect with discord.
    channel_ids: Vec<ChannelId>,    // Channels the bot listens on, definsed in botconfig.toml.
}

impl TypeMapKey for Config {
    type Value = Config;
}

fn main() -> Result<()> {
    let config: Config = loadconfig().expect("Can't load configfile botconfig.toml, Please make sure you have one next to the executable and it's correct.");
    println!("Botconfig loaded {:?}", &config);

    let mut client = Client::new(&config.own_bot_token, Handler).expect("Error creating client");

    // Add config data to bot context so it becomes accessible throughout the bot.
    // It is in a separate scope to drop the mutable reference right after we add the config. 
    {
        let mut data = client.data.write();
        data.insert::<Config>(config);
    }

    // Finally, start a single shard, and start listening to events.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
    Ok(())
}

// Unit struct, so we can implement an EventHandler.
struct Handler;

impl EventHandler for Handler {
    // Messages from all channels where the bot is present. Good practice to first filter out messages we are not interested in.
    fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read();
        let config = data
            .get::<Config>()
            .expect("Expected Config in SharedMap, Please check your botconfig.toml");
        
        // First filter, only predefined ChannelID-s will be checked further.
        if config.channel_ids.contains(&msg.channel_id) {
            // --howtohack function returns information and resources.
            if msg 
                .content
                .starts_with(&format!("{}howtohack ", &config.marker))
            {
                let mut message1 = msg
                    .channel_id
                    .say(
                        &ctx, 
                        format!("Steps:\n1) Open google with triple proxy and triple tor from behind a double vpn\n2) Type in `http://www.hackertyper.com/`\n3) ???\n4) Success")
                    )
                    .unwrap();

                let mut message2 = msg
                    .channel_id
                    .say(
                        &ctx,
                        format!("https://tenor.com/view/typing-fast-computer-silly-gif-14629981"),
                    )
                    .unwrap();

                std::thread::sleep(std::time::Duration::from_millis(7000));

                let _ = message1.edit(&ctx, |m| {
                    m.content("");
                    m.embed(|e| {
                        e.title("Starting with hacking");

                        e.description("We're perfectly happy to help if you come with direct question where you state what you want to accomplish. This being said, you have to do the bulk of the work yourself. If you're looking for someone who will take you by the hand and teach you how to hack, then you're in the wrong place. In fact, I think you'll have a very hard time finding anyone in the hacker community that is willing to do that, because learning by yourself is in the very backbone of the community.\n\n* If you need tips for resources on specific topics, feel free to ask\n* If you need help solving a specific task, then feen free to ask (but do try yourself first)\n* If you need help understanding something, feel free to ask\n**But you have to be the driving force, and you have to do the learning.**\n**We won't teach you.**");

                        e.field("**Hacking is self initiative**\n", "•You should teach yourself and once you can't do  something or achieve something within that field come with questions.\n•Reality of it is, nobody will spend their valuable time mentoring someone who hasn't showed the motivation to teach himself/herself.\n•Hacking isn't a hobby/profession that will take you by the hand to walk through it.\n•It's mostly self initiative and motivation to achieve something.\n•We're ready to help you with a question related to a specific field, however we really aren't the people atleast I am not that will take you by the hand to walk you through.", false);

                        e.field("**You cannot learn everything there is to know about hacking**", "There's not a jack of all trades course, you pick up something and start learning bit by bit, whenever you're stuck after doing proper research people can provide advice and guidance if that's the case", false);

                        e.field("**War Games**", "There is a list of war games.\n•Get some books, and read, watch some online tutorials.\n•You can also do some vulnhub if you can't get into HackTheBox right away.\n•There are many writeups available to see how others have done some vms.\n•The biggest resource will be google , get used to searching for the correct terms , and also how to effectively search on Google , 80% will be googling exploits etc.\n•And lastly, enumerate, the most important step is enumeration.", false);

                        e.field("**List Of Wargames**", "[Click for a list of wargames](https://cdn.discordapp.com/attachments/429676012129746944/654913326266187808/wargames-1.md)", false);

                        e.field("**You cannot learn everything there is to know about hacking**", "There's not a jack of all trades course, you pick up something and start learning bit by bit, whenever you're stuck after doing proper research people can provide advice and guidance if that's the case", false);

                        e.image("https://cdn.discordapp.com/attachments/429676012129746944/720293243434303578/unknown.png");

                        e.colour(Colour::DARK_TEAL);

                        e.footer(|f| {
                            f.text("WhiteHat Hacking https://discord.gg/whAx4qh");

                            f
                        });
                        e
                    });
                    m
                });

                let _ = message2.edit(&ctx, |m| {
                    m.content("");
                    m.embed(|e| {
                        e.title("Resources");

                        e.fields(vec![
                            ("Resources to learn Js", "**Documentation:**\n[MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript)\n[jQuery](http://contribute.jquery.org/documentation/)\n[NodeJS](https://nodejs.org/en/docs/)\n[Typescript](https://www.typescriptlang.org/docs/home.html)\n[Discord.js](https://discord.js.org/#/docs/main/stable/general/welcome)\n**Tutorials:**\n[Eloquent](http://eloquentjavascript.net/)\n[You Don't Know JS](https://github.com/getify/You-Dont-Know-JS)\n[MDN](https://developer.mozilla.org/en-US/docs/Learn/JavaScript/First_steps)\n[Modern JS](http://javascript.info/)\n[Evie's Accelerated JS Tutorial](https://evie.gitbook.io/js/)\n[JavaScript Promises for Dummies](https://scotch.io/tutorials/javascript-promises-for-dummies)\n**Other:**\n[You might not need jQuery](http://youmightnotneedjquery.com/)\n[You might not need jQuery 2](https://github.com/you-dont-need-x/you-dont-need-jquery)\n[CodingTrain | tutorials with examples using p5.js](https://www.youtube.com/user/shiffman)", true),

                            ("Resources to learn C", "**Books:**\n[C Programming Language, 2nd Edition](https://www.amazon.com/dp/0131103628)\n[C Programming: A Modern Approach, 2nd Edition](https://www.amazon.com/dp/0393979504)\n[C Programming Absolute Beginner's Guide, 3rd Edition](https://www.amazon.com/dp/0789751984)\n[Programming in C (4th Edition)](https://www.amazon.com/dp/0321776410)\n[Expert C Programming - Deep secrets](https://www.amazon.ca/dp/0131774298)\n[Modern C by Jens Gustedt](https://gforge.inria.fr/frs/download.php/latestfile/5298/ModernC.pdf)\n**Online resources:**\n[Learn C](http://www.learn-c.org/)\n[C Tutorial](http://crasseux.com/books/ctutorial/)\n[C Boot Camp](https://www.gribblelab.org/CBootCamp)\n[C Programming Guide](https://web.archive.org/web/20170829225943/)\n[C Programming Guide](https://classes.soe.ucsc.edu/cmpe013/Spring10/notes/C%20Programming%20Guide.pdf)\n[FAQ](http://c-faq.com/)\n[Documentation:](http://en.cppreference.com/w/c)", true),

                            ("Resources to learn Python", "**Beginner:**\n[Official tutorial](https://docs.python.org/3/tutorial/)\n[Automate the boring stuff](https://automatetheboringstuff.com/)\n[Guide for beginners](https://wiki.python.org/moin/BeginnersGuide/NonProgrammers)\n**Documentation:**\n[Python 3 Documentation](https://docs.python.org/3/)\n**Videos:**\n[Socratica](https://goo.gl/8xKVKE)\n[Python for beginners](https://www.youtube.com/watch?v=jFCNu1-Xdsw&list=PLlrxD0HtieHhS8VzuMCfQD4uJ9yne1mE6)\n[More python for beginners](https://www.youtube.com/watch?v=xYDnHi0u6_c&list=PLlrxD0HtieHiXd-nEby-TMCoUNwhbLUnj)\n**Free Books:**\n[Books](https://goo.gl/Lxhp7i)\n**Not Free:**\n[Python Crash Course](https://goo.gl/XQ7Nx6)\n[Advanced](https://wiki.python.org/moin/AdvancedBooks)\n**Other:**\n[TalkPython | Podcast](https://goo.gl/xwieUA)\n[Exercises](http://www.practicepython.org/)\n[List Comprehensions](https://www.programiz.com/python-programming/list-comprehension)\n[More Resources](https://goo.gl/Lw3Vqi)", true),

                            ("Resources to learn Java", "**Getting started with Java:**\n[Download](https://jdk.java.net/)\n[University of Finland MOOC](https://java-programming.mooc.fi/)\n[Oracle’s Java tutorial](https://docs.oracle.com/javase/tutorial/index.html)\n[Documentation](https://docs.oracle.com/en/java/javase/14/docs/api/index.html)\n[API Version differences](https://javaalmanac.io/)\n**Alternative JDK Builds:**\n[Oracle](https://www.oracle.com/java/technologies/javase-downloads.html)\n[OpenJDK](https://adoptopenjdk.net/)", true),

                            ("Resources to learn Go", "[Discuss the Go programming language](https://golang.org/)\n[An Interactive Tour of Go](https://tour.golang.org/welcome/1)\n[Go by Example](https://gobyexample.com/)\n[How to Write Go Code](https://golang.org/doc/code.html)\n[Go Language Reference](https://golang.org/ref/spec)\n[Effective Go](https://golang.org/doc/effective_go.html)", true),

                            ("Resources to learn Rust", "[Learn Rust](https://doc.rust-lang.org/book/)\n[Play with Rust](https://play.rust-lang.org/)\n[Crates](https://crates.io/)\n[Learn Rust with Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/)\n[Awesome Rust](https://github.com/rust-unofficial/awesome-rust)\n[Rust](https://www.arewewebyet.org/)\n[Rust Async](https://areweasyncyet.rs/)\n[Rust Blog](https://blog.rust-lang.org/)\n[Intro to Rust Video](https://www.youtube.com/watch?v=EYqceb2AnkU&list=PLJbE2Yu2zumDF6BX6_RdPisRVHgzV02NW&pbjreload=101)", true),

                        ]);

                        e.colour(Colour::DARK_TEAL);

                        e.footer(|f| {
                            f.text("WhiteHat Hacking https://discord.gg/whAx4qh");
                            f
                        });
                        e
                    });
                    m
                });
            // --google function simply creates a lmgtfy link with the given arguments.
            } else if msg 
                .content
                .starts_with(&format!("{}google ", &config.marker))
            {
                let base = "https://lmgtfy.com/?q={}&s=d";
                let toencode = msg
                    .content
                    .replace(&format!("{}google ", &config.marker), "");
                let encoded = encode(&toencode);
                let tosend = base.replace("{}", &encoded);
                if let Err(why) = msg.channel_id.say(&ctx, tosend) {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }
    // Bot ready event.
    fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

// Loading bot config file.
fn loadconfig() -> Result<Config> {
    let configtoml = std::fs::read_to_string("botconfig.toml")?;
    let decoded: Config = toml::from_str(&configtoml)?;
    Ok(decoded)
}
