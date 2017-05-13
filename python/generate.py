#!/usr/bin/python3

import argparse
import sys
from langgen import generate


def main(args):
    parser = argparse.ArgumentParser()
    parser.add_argument("fname", help="text file for corpus")
    parser.add_argument("-n", "--number", type=int, default=1,
                        help="number of random sequences")
    args = parser.parse_args(args)

    print(generate(args.fname, args.number))


if __name__ == '__main__':
    main(sys.argv[1:])
