pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "wirewave-android"

include(":core:config")
project(":core:config").projectDir = file("../../core/config/android")

include(":core:identity")
project(":core:identity").projectDir = file("../../core/identity/android")

include(":core:compatibility")
project(":core:compatibility").projectDir = file("../../core/compatibility/android")

include(":core:updater")
project(":core:updater").projectDir = file("../../core/updater/android")

include(":core:crypto")
project(":core:crypto").projectDir = file("../../core/crypto/android")

include(":core:transport")
project(":core:transport").projectDir = file("../../core/transport/android")

include(":core:discovery")
project(":core:discovery").projectDir = file("../../core/discovery/android")

include(":core:session")
project(":core:session").projectDir = file("../../core/session/android")

include(":features:settings")
project(":features:settings").projectDir = file("../../features/settings/android")

include(":features:connection")
project(":features:connection").projectDir = file("../../features/connection/android")

include(":core:stream")
project(":core:stream").projectDir = file("../../core/stream/android")

include(":features:speaker")
project(":features:speaker").projectDir = file("../../features/speaker/android")

include(":features:mic")
project(":features:mic").projectDir = file("../../features/mic/android")

include(":app")
project(":app").projectDir = file("app")
