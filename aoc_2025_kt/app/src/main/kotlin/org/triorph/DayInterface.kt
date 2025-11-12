package org.example

interface DayInterface<T> {
    fun calculatePartA(): T

    fun calculatePartB(): T

    val name: String
}
