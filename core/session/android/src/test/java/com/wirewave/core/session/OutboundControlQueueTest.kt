package com.wirewave.core.session

import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class OutboundControlQueueTest {
    @Test
    fun `speaker command is available immediately without an inbound heartbeat`() = runBlocking {
        val queue = OutboundControlQueue()

        assertTrue(queue.trySendSpeaker(active = true))
        val command = withTimeout(100) { queue.commands.receive() }

        assertEquals(OutboundControlCommand.Speaker(active = true), command)
    }

    @Test
    fun `mic command is available immediately without an inbound heartbeat`() = runBlocking {
        val queue = OutboundControlQueue()

        assertTrue(queue.trySendMic(active = true))
        val command = withTimeout(100) { queue.commands.receive() }

        assertEquals(OutboundControlCommand.Mic(active = true), command)
    }
}
