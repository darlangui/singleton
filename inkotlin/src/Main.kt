import db.DatabaseConnection
import utils.Logger
import lazy.singlelazy

fun databaseExample() {
    println("Initializing bd")

    val conn = DatabaseConnection.connect()

    val stmt = conn.createStatement()
    val rs = stmt.executeQuery("SELECT * FROM users")

    val pstmt = conn.prepareStatement("INSERT INTO users (id, name) VALUES (?, ?)")
    pstmt.setInt(1, 3)
    pstmt.setString(2, "Bob Johnson")
    val rows = pstmt.executeUpdate()
    println("$rows linhas inseridas")

    DatabaseConnection.disconnect()

    val conn2 = DatabaseConnection.connect()
    println("Conexão 2: $conn2")

    DatabaseConnection.disconnect()
    println("Disconnect bd")
}

fun lazyInitializationExample() {
    singlelazy.instance.manage()

    val single = singlelazy.instance

    println(single === singlelazy.instance)
}

fun loggerExample() {
    Logger.info("Initialized")

    try {
        Logger.debug("Process")
        throw RuntimeException("Error in Process...")
    } catch (e: Exception) {
        Logger.error("Failed in Process...", e)
    }
}

fun main() {
    databaseExample()
    lazyInitializationExample()
    loggerExample()
}