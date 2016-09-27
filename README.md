# lurk
(CLI)
Displays text files, optionally filtered by search terms.

As this is my first real rust project any help or tips on how to make the code more efficient is appreciated.



TODO:
*Create a makefile for this project to be able to install it, currently users have to add the binary to a $PATH variable themselves or copy it to a location that is already in the $PATH.

*Add a optional limiter to chose how many lines are displayed.

*Somehow read in what text color is being used in the console as lurk currently assumes white and will leave the console text color as white after usage.

*Add more colors to the search module and possibly a way to have a config file where the order of them is specified.


Uses external crates:
clap - latest available version.
