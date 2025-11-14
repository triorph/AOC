package org.triorph

import io.kotest.matchers.shouldBe
import kotlin.test.Test

class Day01Test {
    @Test
    fun `Test day01 part A result`() {
        val day01 = Day01()
        day01.calculatePartA() shouldBe 0
    }

    @Test
    fun `Test day01 part B result`() {
        val day01 = Day01()
        day01.calculatePartB() shouldBe 0
    }
}
