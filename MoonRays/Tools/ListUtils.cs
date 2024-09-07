namespace MoonRays.Tools;

public static class ListUtils
{
    public static int IndexOfMax(List<long> list)
    {
        if (list == null || list.Count == 0)
        {
            throw new ArgumentException("The list cannot be null or empty.");
        }

        long maxValue = list[0];
        int maxIndex = 0;

        for (int i = 1; i < list.Count; i++)
        {
            if (list[i] > maxValue)
            {
                maxValue = list[i];
                maxIndex = i;
            }
        }

        return maxIndex;
    }
}