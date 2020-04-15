#!/usr/bin/env python

import sys
import os

sys.path.append(os.path.join(os.path.dirname(__file__), "..", "..", "libs", "$name_snake_case$-client-py"))

import $name_snake_case$

def main():

    target_$param.service_name_snake_case$s = [
        ["Zufar", "zufar@mail.com", "+628123123"],
        ["Akmal", "akmal@mail.com", "+628123124"],
        ["Anto", "anto@mail.com", "+628123125"],
        ["Hanky", "hanky@mail.com", "+628123126"],
        ["Andrie", "andrie@mail.com", "+628123127"],
        ["Ubai", "ubai@mail.com", "+628123128"],
    ]

    tokens = []
    for acc in target_$param.service_name_snake_case$s:
        tokens.append($name_snake_case$.register_$param.service_name_snake_case$(acc[0], acc[1], acc[2]))

    print(tokens)

    for token in tokens:
        if isinstance(token, str):
            if $name_snake_case$.activate_$param.service_name_snake_case$(token, "123").status_code != 200:
                print("cannot activate $param.service_name$ with token %s", token)
        

if __name__ == "__main__":
    main()
