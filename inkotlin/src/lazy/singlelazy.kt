package lazy

class singlelazy private constructor() {
    companion object {
        val instance by lazy {
            singlelazy()
        }
    }

    fun manage() {
        println("Manage called")
    }
}