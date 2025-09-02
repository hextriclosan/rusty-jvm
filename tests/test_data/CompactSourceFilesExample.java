void main() {
    var authors = List.of("James", "Bill", "Guy", "Alex", "Dan", "Gavin");
    for (var name : authors) {
        IO.println(name + ": " + name.length());
    }
}
