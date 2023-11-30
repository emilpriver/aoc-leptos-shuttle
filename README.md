# AdventOfCode Leaderbord Leptos

AdventOfCode Leaderbord built in leptos and rust. 
It is refetching the leaderbord every 15 minutes

## Run

```bash
docker run -e LEADERBOARD_ID=X -e SESSION_COOKIE_TOKEN=X emilpriver/aoc-leaderbord:latest
```

## Environment variables

### SESSION_COOKIE_TOKEN (Required)
This is the value of the cookie `session` you can fetch when you login to https://adventofcode.com/ and browse cookies.

### LEADERBOARD_ID (Required)
This is the ID of the Leaderbord and can be found in the url when you browse your Leaderbord


### LEADERBOARD_YEAR (Optional)
Leaderbord year, can be used if you want to show the leaderbord for a older year. Default to `2023`
