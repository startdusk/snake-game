#!/bin/bash

wasm-pack build --target web && npm run heroku-prebuild && npm run build && npm start