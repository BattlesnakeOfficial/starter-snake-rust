## Run boa

```sh
cargo run
```

You should see the following output once it is running

```sh
🚀 Rocket has launched from http://0.0.0.0:8000
```

Open [localhost:8000](http://localhost:8000) in your browser and you should see

```json
{"apiversion":"1","author":"","color":"#888888","head":"default","tail":"default"}
```

## Play a Game Locally

Install the [Battlesnake CLI](https://github.com/BattlesnakeOfficial/rules/tree/main/cli)
* You can [download compiled binaries here](https://github.com/BattlesnakeOfficial/rules/releases)
* or [install as a go package](https://github.com/BattlesnakeOfficial/rules/tree/main/cli#installation) (requires Go 1.18 or higher)

Command to run a local game

```sh
battlesnake play -W 11 -H 11 --name 'Rust Starter Project' --url http://localhost:8000 -g solo --browser
```

## Github action

When something got merged to `main` a Github action will publish to production.

## TO-DO

Lots of things are pending, this is a very unusable / not-competitive snake that I use to learn Rust and infrastructure stuff
 - Warped maps
 - The minimize part of minimax
 - Move opponent snakes during minimax (currently had been tested in solo mode)
 - Improve resource usage (par_map does a great job, but there is a lot of place for improvement)
 - Prepare a an staging snake (perhaps with a Github Action looking at staging branch?)
 - Prepare an infrastructure branch for automation the creation of Systemd services / environments 

## More info on battlesnake

Continue with the [Battlesnake Quickstart Guide](https://docs.battlesnake.com/quickstart) to customize and improve your Battlesnake's behavior.

**Note:** To play games on [play.battlesnake.com](https://play.battlesnake.com) you'll need to deploy your Battlesnake to a live web server OR use a port forwarding tool like [ngrok](https://ngrok.com/) to access your server locally.
