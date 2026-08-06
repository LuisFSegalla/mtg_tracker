# Magic the Gathering stats tracker

An app for tracking results in Modern and Pauper games. You can save matches and get stats based on each deck you added.

Creates a database for loading the stats and match history from and utilizes an intuitive terminal UI for updating games.

For running locally you can either run as a devcontainer in VSCode or directly from the terminal by building and running the app locally.

To build:
```console
docker build --target runtime -t mtg_tracker .
```

The build will result in a `image_id` that you can find with:

```console
docker images
```
Use the `image_id` to run the application with: 

```console
docker run -it <image id>
```