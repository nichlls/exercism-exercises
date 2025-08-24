public static class DialingCodes
{
    public static Dictionary<int, string> GetEmptyDictionary() => [];

    // "United States of America" which has a code of 1, "Brazil" which has a code of 55 and "India" which has a code of 91
    public static Dictionary<int, string> GetExistingDictionary()
    {
        return new Dictionary<int, string>
        {
            { 1, "United States of America" },
            { 55, "Brazil" },
            { 91, "India" },
        };
    }

    public static Dictionary<int, string> AddCountryToEmptyDictionary(
        int countryCode,
        string countryName
    ) => new() { { countryCode, countryName } };

    public static Dictionary<int, string> AddCountryToExistingDictionary(
        Dictionary<int, string> existingDictionary,
        int countryCode,
        string countryName
    ) =>
        new()
        {
            { 1, "United States of America" },
            { 55, "Brazil" },
            { 91, "India" },
            { countryCode, countryName },
        };

    public static string GetCountryNameFromDictionary(
        Dictionary<int, string> existingDictionary,
        int countryCode
    )
    {
        var country = existingDictionary.FirstOrDefault(x => x.Key == countryCode);
        if (country.Value == null)
        {
            return string.Empty;
        }
        return country.Value;
    }

    public static bool CheckCodeExists(
        Dictionary<int, string> existingDictionary,
        int countryCode
    ) => existingDictionary.ContainsKey(countryCode);

    public static Dictionary<int, string> UpdateDictionary(
        Dictionary<int, string> existingDictionary,
        int countryCode,
        string countryName
    )
    {
        if (existingDictionary.ContainsKey(countryCode))
        {
            existingDictionary[countryCode] = countryName;
            return existingDictionary;
        }
        else
        {
            return existingDictionary;
        }
    }

    public static Dictionary<int, string> RemoveCountryFromDictionary(
        Dictionary<int, string> existingDictionary,
        int countryCode
    )
    {
        existingDictionary.Remove(countryCode);
        return existingDictionary;
    }

    public static string FindLongestCountryName(Dictionary<int, string> existingDictionary)
    {
        var longest = 0;
        var countryName = "";

        foreach (var item in existingDictionary)
        {
            if (item.Value.Length > longest)
            {
                longest = item.Value.Length;
                countryName = item.Value;
            }
        }
        return countryName;
    }
}
