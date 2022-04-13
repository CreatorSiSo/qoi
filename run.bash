#!/bin/sh

cargo run -q && hexyl --border none ./result.qoi | bat --file-name result.qoi
