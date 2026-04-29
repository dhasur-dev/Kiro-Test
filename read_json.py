#!/usr/bin/env python3
"""Read and parse a JSON file from a given path."""

import argparse
import json
import sys


def read_json(path):
    """Read and parse a JSON file at the given path.

    Args:
        path: Path to the JSON file.

    Returns:
        The parsed JSON content.
    """
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def main():
    parser = argparse.ArgumentParser(
        description="Read and parse a JSON file from a given path."
    )
    parser.add_argument("path", help="Path to the JSON file")
    args = parser.parse_args()

    try:
        data = read_json(args.path)
    except FileNotFoundError:
        print(f"Error: file not found: {args.path}", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"Error: invalid JSON in {args.path}: {e}", file=sys.stderr)
        sys.exit(1)
    except OSError as e:
        print(f"Error: could not read {args.path}: {e}", file=sys.stderr)
        sys.exit(1)

    print(json.dumps(data, indent=2))


if __name__ == "__main__":
    main()
