plugins {
    kotlin("jvm") version "2.0.0"
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.aallam.openai:openai-client:3.7.2")

    val ktor_version = "2.3.11"
    implementation("io.ktor:ktor-client-cio:$ktor_version")
    implementation("io.ktor:ktor-client-content-negotiation:$ktor_version")
    implementation("io.ktor:ktor-serialization-kotlinx-json:2$ktor_version")

    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.9.0-RC")
}

application {
    mainClass.set("GenerateNewsKt")
}
