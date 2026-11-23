package com.wirewave.features.speaker

import java.util.TreeMap

enum class LatencyMode(val targetMs: Int, val label: String) {
    LOW(40, "Low (40 ms)"),
    BALANCED(80, "Balanced (80 ms)"),
    STABLE(140, "Stable (140 ms)")
}

/**
 * Sequence-aware UDP jitter buffer.
 * Reorders minor out-of-order packets, prerolls to [targetPackets], drops excessive
 * backlog to cap latency, and outputs smooth silence on packet loss.
 */
class JitterBuffer(initialMode: LatencyMode = LatencyMode.BALANCED) {
    private val lock = Any()
    private val queue = TreeMap<Long, ByteArray>()
    private val silencePacket = ByteArray(LaudPacket.PCM_BYTES_PER_PACKET)
    private var prerolling = true
    private var nextSeq: Long? = null
    @Volatile
    private var targetPackets: Int = msToPackets(initialMode.targetMs)

    fun setLatencyMode(mode: LatencyMode) {
        targetPackets = msToPackets(mode.targetMs)
    }

    fun push(packet: LaudPacket) {
        synchronized(lock) {
            val expected = nextSeq
            if (expected != null && packet.sequence < expected) {
                return
            }
            queue[packet.sequence] = packet.pcmBytes
            val maxPackets = (targetPackets * 3).coerceAtLeast(16)
            while (queue.size > maxPackets) {
                val dropped = queue.pollFirstEntry()
                nextSeq = dropped?.key?.plus(1L)
            }
            if (prerolling && queue.size >= targetPackets) {
                prerolling = false
                nextSeq = queue.firstKey()
            }
        }
    }

    fun popNext(): ByteArray {
        synchronized(lock) {
            if (prerolling) return silencePacket
            val seq = nextSeq ?: return silencePacket
            val exact = queue.remove(seq)
            if (exact != null) {
                nextSeq = seq + 1L
                return exact
            }
            if (queue.isEmpty()) {
                prerolling = true
                nextSeq = null
                return silencePacket
            }
            // Packet lost: advance past the gap smoothly instead of stalling.
            nextSeq = seq + 1L
            return silencePacket
        }
    }

    fun clear() {
        synchronized(lock) {
            queue.clear()
            prerolling = true
            nextSeq = null
        }
    }

    companion object {
        private const val MS_PER_PACKET = 5.333f
        private fun msToPackets(ms: Int): Int = (ms / MS_PER_PACKET).toInt().coerceIn(4, 40)
    }
}
