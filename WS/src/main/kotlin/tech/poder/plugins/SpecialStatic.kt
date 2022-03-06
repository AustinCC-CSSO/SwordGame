package tech.poder.plugins

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.http.content.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.util.*
import java.io.File
import java.nio.file.Files
import java.nio.file.Paths

object SpecialStatic {
    private const val pathParameterName = "static-content-path-parameter"

    fun Route.resources(resourcePackage: String) {
        val packageName = staticBasePackage.combinePackage(resourcePackage)
        get("{$pathParameterName...}") {
            val relativePath = call.parameters.getAll(pathParameterName)?.joinToString(File.separator) ?: return@get
            val content = call.resolveResource(relativePath, packageName)
            if (content != null) {
                call.respond(content)
            }
        }
    }

    fun Route.extras(baseLocation: String) {
        get("{$pathParameterName...}") {
            val relativePath = call.parameters.getAll(pathParameterName)?.joinToString(File.separator) ?: return@get
            val path = Paths.get(baseLocation, relativePath)
            if (Files.exists(path)) {
                val content = LocalFileContent(path.toFile(), ContentType.defaultForFileExtension(path.extension))
                call.respond(content)
            }
        }
    }

    private fun String?.combinePackage(resourcePackage: String) = when {
        this == null -> resourcePackage
        else -> "$this.$resourcePackage"
    }
}