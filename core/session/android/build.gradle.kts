plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.wirewave.core.session"
    compileSdk = 34
    defaultConfig { minSdk = 30 }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }
}

dependencies {
    implementation(project(":core:config"))
    implementation(project(":core:identity"))
    implementation(project(":core:crypto"))
    implementation(project(":core:transport"))
    implementation(project(":core:discovery"))
    implementation(project(":core:stream"))
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.7.3")
    testImplementation("junit:junit:4.13.2")
}
