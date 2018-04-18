#! /usr/bin/env bash

cargo wasm build
ls site
npm test
