public static class LogAnalysis
{
    // TODO: define the 'SubstringAfter()' extension method on the `string` type
    public static string SubstringAfter(this string str, string delimit)
    {
        string[] newString = str.Split(delimit);
        return newString[^1];
    }

    // TODO: define the 'SubstringBetween()' extension method on the `string` type
    public static string SubstringBetween(this string str, string delimit1, string delimit2)
    {
        var delimit1Index = str.IndexOf(delimit1);
        var delimit2Index = str.IndexOf(delimit2);
        var startIndex = delimit1.Length + delimit1Index;

        return str[startIndex..delimit2Index];
    }

    // TODO: define the 'Message()' extension method on the `string` type
    public static string Message(this string str) => SubstringAfter(str, ": ");

    // TODO: define the 'LogLevel()' extension method on the `string` type
    public static string LogLevel(this string str) => SubstringBetween(str, "[", "]");
}
