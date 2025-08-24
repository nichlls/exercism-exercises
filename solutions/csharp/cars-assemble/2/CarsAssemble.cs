static class AssemblyLine
{
    public static double SuccessRate(int speed)
    {
        if (speed == 0)
            return 0.00;
        if (speed <= 4)
            return 1.00;
        if (speed <= 8)
            return 0.90;
        if (speed == 9)
            return 0.80;
        return 0.77;
    }

    public static double ProductionRatePerHour(int speed) => speed * 221 * SuccessRate(speed);

    public static int WorkingItemsPerMinute(int speed) => (int)(ProductionRatePerHour(speed) / 60);
}
