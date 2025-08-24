public static class PhoneNumber
{
    public static (bool IsNewYork, bool IsFake, string LocalNumber) Analyze(string phoneNumber)
    {
        string[] splitNumber = phoneNumber.Split("-");
        bool IsNewYork = splitNumber[0] == "212";
        bool isNumberFake = splitNumber[1] == "555";

        return (IsNewYork, isNumberFake, splitNumber[2]);
    }

    public static bool IsFake((bool IsNewYork, bool IsFake, string LocalNumber) phoneNumberInfo)
    {
        return phoneNumberInfo.IsFake;
    }
}
