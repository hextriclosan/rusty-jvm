// build command:
// javac -g -parameters Mutf8.java

interface Mutf8 {
    String withZero = "\0abc";
    String singleByteLatin = "A";
    String twoByteUkrainian = "ї";
    String threeByteSnowman = "☃";
    String fourByteGothicLetterHwair = "𐍈";
    String fourByteEmoji = "😂";
    String withNonValidSequences = "a\ud800\ud800💔\ud800b";
}
