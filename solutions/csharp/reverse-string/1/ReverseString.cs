public static class ReverseString
{
    public static string Reverse(string input)
    {
        char[] charInput = input.ToCharArray();
        Array.Reverse(charInput);
        return new string(charInput);
    }
}