package db

object DatabaseConnection {
    private const val URL = "jdbc:postgresql://localhost:5432/postgres"
    private const val USER = "postgres"
    private const val PASSWORD = "postgres"
    private const val DRIVER = "org.postgresql.Driver"

    private var connection: MockConnection? = null

    init {
        // Simulando carregamento do driver
        // Class.forName(DRIVER)
        println("Initializing Database in $URL")
    }

    fun connect(): MockConnection {
        if (connection == null) {
            connection = MockConnection(URL, USER, PASSWORD)
            println("Connected to $URL")
        } else {
            println("Using existing connection to $URL")
        }

        return connection!!
    }

    fun disconnect() {
        connection?.let {
            it.close()
            connection = null
            println("Disconnected from $URL")
        }
    }
}

class MockConnection(
    private val url: String,
    private val user: String,
    private val password: String
) {
    private var closed = false

    fun isClosed(): Boolean = closed

    fun close() {
        closed = true
    }

    fun createStatement(): MockStatement {
        return MockStatement()
    }

    fun prepareStatement(sql: String): MockPreparedStatement {
        return MockPreparedStatement(sql)
    }

    override fun toString(): String {
        return "MockConnection(url='$url', user='$user')"
    }
}

class MockStatement {
    fun executeQuery(sql: String): MockResultSet {
        println("Executing query: $sql")
        return MockResultSet()
    }

    fun executeUpdate(sql: String): Int {
        println("Executing update: $sql")
        return 1
    }
}

class MockPreparedStatement(private val sql: String) {
    fun setInt(parameterIndex: Int, value: Int) {
        println("Setting parameter $parameterIndex to $value")
    }

    fun setString(parameterIndex: Int, value: String) {
        println("Setting parameter $parameterIndex to '$value'")
    }

    fun executeQuery(): MockResultSet {
        println("Executing prepared query: $sql")
        return MockResultSet()
    }

    fun executeUpdate(): Int {
        println("Executing prepared update: $sql")
        return 1
    }
}

class MockResultSet {
    private var currentIndex = -1
    private val data = listOf(
        mapOf("id" to 1, "name" to "John Doe"),
        mapOf("id" to 2, "name" to "Jane Smith")
    )

    fun next(): Boolean {
        currentIndex++
        return currentIndex < data.size
    }

    fun getInt(columnName: String): Int {
        return data[currentIndex][columnName] as Int
    }

    fun getString(columnName: String): String {
        return data[currentIndex][columnName] as String
    }

    fun close() {
        println("ResultSet closed")
    }
}