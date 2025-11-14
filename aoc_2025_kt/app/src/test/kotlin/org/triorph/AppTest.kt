package org.triorph

import kotlin.test.Test
import kotlin.test.assertNotNull

class AppTest {
    @Test
    fun appHasAResult() {
        val classUnderTest = App()
        assertNotNull(classUnderTest.runAllDays(), "App should return a result")
    }
}
