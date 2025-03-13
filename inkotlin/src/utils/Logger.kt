package utils

object Logger {
    enum class Level {
        DEBUG, INFO, WARN, ERROR
    }

    private var currentLevel: Level = Level.DEBUG

    fun debug(msg: String) {
        if (currentLevel.ordinal <= Logger.Level.DEBUG.ordinal) {
            println("[DEBUG] $msg]")
        }
    }

    fun info(msg: String) {
        if (currentLevel.ordinal <= Logger.Level.INFO.ordinal) {
            println("[INFO] $msg")
        }
    }

    fun warn(msg: String) {
        if (currentLevel.ordinal <= Logger.Level.WARN.ordinal) {
            println("[WARN] $msg")
        }
    }

    fun error(msg: String) {
        if (currentLevel.ordinal <= Logger.Level.ERROR.ordinal) {
            println("[ERROR] $msg")
        }
    }

    fun error(msg: String, throwable: Throwable) {
        error("$msg : ${throwable.message}")
        throwable.printStackTrace()
    }
}