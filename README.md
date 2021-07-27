# A Simple [Battlesnake](http://play.battlesnake.com?utm_source=github&utm_medium=readme&utm_campaign=go_starter&utm_content=homepage) Written in Rust

![Battlesnake Logo](https://media.battlesnake.com/social/GitHubSocial.png)

This is a basic implementation of the [Battlesnake API](https://docs.battlesnake.com/references/api). It's a great starting point for anyone wanting to program their first Battlesnake using Rust.

## Technologies Used

* [Rust](https://www.rust-lang.org/)


## Quickstart

The [Quick Start Coding Guide](https://docs.battlesnake.com/guides/getting-started) provides the full set of instructions to customize, register, and create your first games with your Battlesnake! While the guide optimizes around local development for quick iteratation, you may choose to host it with [Repl.it](https://repl.it) or a provider of your choice. You can find advice on other hosting providers within our [Hosting Suggestions](https://docs.battlesnake.com/references/hosting-suggestions) page.


### Prerequisites

* A free [Battlesnake Account](https://play.battlesnake.com/?utm_source=github&utm_medium=readme&utm_campaign=rust_starter&utm_content=homepage)

---

## Customizing Your Battlesnake

Locate the `snake_info` function inside [logic.rs](src/logic.rs#L9). Inside that function you should see a line that looks like this:

```rust
pub fn snake_info() -> JsonValue {
    return json!({
        "apiversion": "1",
        "author": "",
        "color": "#888888",
        "head": "default",
        "tail": "default",
    });
}
```

This function is called by the game engine periodically to make sure your Battlesnake is healthy, responding correctly, and to determine how your Battlesnake will appear on the game board. See [Battlesnake Personalization](https://docs.battlesnake.com/references/personalization) for how to customize your Battlesnake's appearance using these values.

Whenever you update these values, go to the page for your Battlesnake and select 'Refresh Metadata' from the option menu. This will update your Battlesnake to use your latest configuration and those changes should be reflected in the UI as well as any new games created.

## Changing Behavior

On every turn of each game your Battlesnake receives information about the game board and must decide its next move.

Locate the `get_move` function inside [logic.rs](src/logic.rs#L30). Possible moves are "up", "down", "left", or "right". To start your Battlesnake will choose a move randomly. Your goal as a developer is to read information sent to you about the board and decide where your Battlesnake should move next. This is the code you will want to edit.

See the [Battlesnake Game Rules](https://docs.battlesnake.com/references/rules) for more information on playing the game, moving around the board, and improving your algorithm.

## Running Your Battlesnake Locally

First clone the repository

```shell
git clone https://github.com/martinamps/starter-snake-rust
cd starter-snake-rust
```

Next, because [rocket](https://rocket.rs) requires nightly builds of rust we recommend you use [rustup](https://rustup.rs/) to get started then set the project to use the nightly builds

```shell
rustup override set nightly
```

We recommend you install `watchexec`(https://github.com/watchexec/watchexec) to reload your snake each time you make a change 

```shell
brew install watchexec
```

Finally, run your snake!

```shell
watchexec --restart RUST_LOG=INFO cargo run
```

**Note:** You cannot create games on [play.battlesnake.com](https://play.battlesnake.com) using a locally running Battlesnake unless you install and use a port forwarding tool like [ngrok](https://ngrok.com/).

To get started with ngrok simply:

```
brew install ngrok
ngrok http 8000
```

Then configure your snake to use the provided tunnel URL (eg: `https://3c91ab96xxx.ngrok.io`) in the dashboard!

---

## Playing Battlesnake

### Completing Challenges

If you're looking for the Single Player Mode of Battlesnake, or something to practice with between events, check out [Challenges.](https://docs.battlesnake.com/guides/quick-start-challenges-guide)

### Joining a Battlesnake Arena

Once you've made your Battlesnake behave and survive on its own, you can enter it into the [Global Battlesnake Arena](https://play.battlesnake.com/arena/global) to see how it performs against other Battlesnakes worldwide.

Arenas will regularly create new games and rank Battlesnakes based on their results. They're a good way to get regular feedback on how well your Battlesnake is performing, and a fun way to track your progress as you develop your algorithm.

### Joining a Battlesnake League

Want to get out there to compete and win prizes? Check out the [Quick Start League Guide](https://docs.battlesnake.com/guides/quick-start-league-guide) for information on the how and when of our competitive seasons.

---

## Resources

All documentation is available at [docs.battlesnake.com](https://docs.battlesnake.com), including detailed Guides, API References, and Tips.

You can also join the Battlesnake Developer Community on [Discord](https://play.battlesnake.com/discord?utm_source=github&utm_medium=readme&utm_campaign=go_starter&utm_content=discord). We have a growing community of Battlesnake developers of all skill levels wanting to help everyone succeed and have fun with Battlesnake :)

Check out live Battlesnake events on [Twitch](https://www.twitch.tv/battlesnakeofficial) and see what is happening when on the [Calendar.](https://play.battlesnake.com/calendar?utm_source=github&utm_medium=readme&utm_campaign=go_starter&utm_content=calendar)

Want to contribute to Battlesnake? We have a number of open-source codebases and would love for you to get involved! Check out our page on [Contributing.](https://docs.battlesnake.com/guides/contributing)


## Feedback

**Do you have an issue or suggestions for this repository?** Head over to our [Feedback Repository](https://play.battlesnake.com/feedback?utm_source=github&utm_medium=readme&utm_campaign=go_starter&utm_content=feedback) today and let us know!

