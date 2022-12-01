import java.io.File
import getElfs

fun readInput() = File("input.txt").inputStream().readBytes().toString(Charsets.UTF_8)
fun getElfs(): List<Int> {
    val content = readInput()
    val elfs = content.split('\n').fold(mutableListOf(0)) { elfs, num ->
        if (num.length != 0) {
            elfs[elfs.lastIndex] = elfs.last() + num.toInt()
        } else {
            elfs.add(0)
        }
        
        elfs
    }

    elfs.sortDescending()
    return elfs
}

fun part1(): String {
    val elfs = getElfs()

    return elfs.first().toString()
}

fun part2(): String {
    val elfs = getElfs()

    return elfs.take(3).sum().toString()
}

fun main(args: Array<String>) {
    val part = args.get(0)

    if (part == "1") {
        val result = part1()
        println(result)
    } else if (part == "2") {
        val result = part2()
        println(result)
    }
}
