#!/usr/bin/env bash

npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch & dx serve --hot-reload --platform desktop

