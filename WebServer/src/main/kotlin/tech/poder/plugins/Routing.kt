package tech.poder.plugins

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.http.content.*
import io.ktor.server.plugins.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import tech.poder.plugins.SpecialStatic.extras
import tech.poder.plugins.SpecialStatic.resources

fun Application.configureRouting() {
    routing {
        get("/") {
            call.respondText("Hello World!")
        }
        // Static plugin. Try to access `/static/index.html`
        static("/") {
            resources("www")
            extras("www")
        }
        install(StatusPages) {
            exception<AuthenticationException> { call, cause ->
                call.respond(HttpStatusCode.Unauthorized)
            }
            exception<AuthorizationException> { call, cause ->
                call.respond(HttpStatusCode.Forbidden)
            }
            statusFile(
                HttpStatusCode.NotFound,
                HttpStatusCode.Unauthorized,
                HttpStatusCode.Forbidden,
                filePattern = "/errors/#.html"
            )
        }
    }
}

class AuthenticationException : RuntimeException()
class AuthorizationException : RuntimeException()
