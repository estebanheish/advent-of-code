#!/usr/bin/env nu
wl-paste | str trim | lines | parse "{left}   {right}" | values | each {into int | sort} | do {|l| $l.0 | zip $l.1 | each {|| $in.0 - $in.1 | math abs } | math sum } $in | print $in
wl-paste | str trim | lines | parse "{left}   {right}" | values | each {into int | sort} | do {|b| $b.0 | each {|l| $l * ($b.1 | filter {|| $in == $l } | length) } } $in | math sum
