import pathlib
import sys
import urllib.request

from env import SESSION_COOKIE

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("bad invocation - make sure to include the day # and no more")
        sys.exit(1)
    day = sys.argv[1]
    input_url = f"https://adventofcode.com/2022/day/{day}/input"

    request = urllib.request.Request(
        input_url,
        headers={
            "Cookie": f"session={SESSION_COOKIE}"
        },
    )

    with urllib.request.urlopen(request) as resp:
        dayN_dir = pathlib.Path(f"./day{day}")
        dayN_dir.mkdir(parents=True, exist_ok=True)
        dayN_input = dayN_dir / "input.txt"
        dayN_input.write_text(resp.read().decode("utf-8"))
