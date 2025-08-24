class RemoteControlCar
{
    private int distanceIncrement = 20;
    private int distanceDriven = 0;
    private int batteryPercent = 100;

    public static RemoteControlCar Buy() => new();

    public string DistanceDisplay() => $"Driven {distanceDriven} meters";

    public string BatteryDisplay()
    {
        return batteryPercent == 0 ? "Battery empty" : $"Battery at {batteryPercent}%";
    }

    public void Drive()
    {
        if (batteryPercent > 0)
        {
            batteryPercent -= 1;
            distanceDriven += distanceIncrement;
        }
        else
        {
            Console.WriteLine(BatteryDisplay());
        }
    }
}
