package main

import java.io.File

fun main() {
    val input = File("input.txt").readText().trim().split("\n")
    var pos = Pair(1, 1)
    var out: Double = 0.0
    for ((i, line) in input.withIndex()) {
        for (char in line) {
            when (char) {
                'D' -> pos = Pair(pos.first, minOf(pos.second + 1, 2))
                'U' -> pos = Pair(pos.first, maxOf(pos.second - 1, 0))
                'R' -> pos = Pair(minOf(pos.first + 1, 2), pos.second)
                'L' -> pos = Pair(maxOf(pos.first - 1, 0), pos.second)
            }
        }
        out += (pos.first + 1 + pos.second * 3) * Math.pow(10.0, (input.size - i - 1).toDouble())
    }
    println(out)
}
