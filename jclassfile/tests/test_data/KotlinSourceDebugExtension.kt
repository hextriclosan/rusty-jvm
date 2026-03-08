inline fun f(block: () -> Unit) = block()

fun main() {
    f { }
}
