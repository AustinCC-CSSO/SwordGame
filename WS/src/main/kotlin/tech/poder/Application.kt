package tech.poder

import io.ktor.server.engine.*
import io.ktor.server.netty.*
import tech.poder.plugins.*

fun main() {
    embeddedServer(Netty, port = 8080, host = "0.0.0.0") {
        configureRouting()
        configureHTTP()
        configureTemplating()
        configureSerialization()
        configureSockets()
    }.start(wait = true)
}
