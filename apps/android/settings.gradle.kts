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

include(":features:settings")
project(":features:settings").projectDir = file("../../features/settings/android")

include(":app")
project(":app").projectDir = file("app")
