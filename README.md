# Host Craft: a personal project to easily host Minecraft servers and play with your friends!
This CLI, used with the propietary JAR file offered by the Mojang company to open a server in a personal computer,
allows to easily host a Minecraft server, play with your friends and centralize the world data in a Git repository.

## About the project
Host Craft is a project built to:
- Learn Rust with self learning
- As a challenge to motivate a young junior developer to create a tool that he can use with its friends.

With the Host Craft CLI tool, any player can:
1. Download the world data of a server given a specific Git repository.
2. Inside that repository, a host file is included. If the server is currently running and hosted
by one of the players, its username will appear in the file. Else, the CLI user can start hosting
the server by itself!
3. Once the user wishes to close the server, the server is closed and all changes made to the world
data is automatically uploaded to the Git repository so anyone can have access to the latest state
of their saved servers!

## Installation and setup