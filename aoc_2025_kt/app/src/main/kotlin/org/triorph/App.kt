package org.triorph

import org.triorph.Day01

class App {
    fun runAllDays(): String =
        allDays
            .map { day ->
                "${day.name} - part A: ${day.calculatePartA()}\n" +
                    "${day.name} - part B: ${day.calculatePartB()}\n"
            }.joinToString()

    companion object {
        private val allDays =
            listOf(
                Day01(),
            )
    }
}

fun main() {
    println(App().runAllDays())
}
