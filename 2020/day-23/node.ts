export default class Node {
    next?: Node
    val: number

    constructor(val: number) {
        this.val = val
        this.next = undefined
    }
}