package org.triorph

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class Day01Test {
    @Test
    fun `Test day01 part A result`() {
        val day01 = Day01()
        assertEquals(day01.calculatePartA(), 0, "App should return a result")
    }

    @Test
    fun `Test day01 part B result`() {
        val day01 = Day01()
        assertEquals(day01.calculatePartB(), 0, "App should return a result")
    }
}
