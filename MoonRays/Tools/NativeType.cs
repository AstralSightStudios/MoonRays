using System.Runtime.InteropServices;
using System.Text;

namespace MoonRays.Tools;

public static class NativeType
{
    public unsafe static byte** ConvertStringListToBytePointerArray(List<string> strings)
    {
        byte[][] byteArrays = new byte[strings.Count][];
        for (int i = 0; i < strings.Count; i++)
        {
            byteArrays[i] = Encoding.UTF8.GetBytes(strings[i]);
        }

        byte** pointerArray = (byte**)Marshal.AllocHGlobal(strings.Count * sizeof(byte*));
        try
        {
            for (int i = 0; i < strings.Count; i++)
            {
                fixed (byte* p = byteArrays[i])
                {
                    pointerArray[i] = p;
                }
            }
        }
        catch
        {
            // 如果在转换过程中发生错误，则释放分配的非托管资源
            Marshal.FreeHGlobal((IntPtr)pointerArray);
            throw;
        }

        return pointerArray;
    }
    
    public static unsafe float** FloatListToFloatPtrPtr(List<float> list)
    {
        float[] array = list.ToArray();
        IntPtr* pointers = (IntPtr*)Marshal.AllocHGlobal(array.Length * sizeof(IntPtr));
        for (int i = 0; i < array.Length; i++)
        {
            float* p = (float*)Marshal.AllocHGlobal(sizeof(float));
            *p = array[i];
            pointers[i] = (IntPtr)p;
        }

        return (float**)pointers;
    }
    
    public static unsafe float* FloatToPtr(float value)
    {
        float* pValue = (float*)Marshal.AllocHGlobal(sizeof(float));
        *pValue = value;

        return pValue;
    }
    
    public static unsafe int FindLength(byte* bytes)
    {
        int length = 0;
        while (bytes[length] != 0)
        {
            length++;
        }
        return length;
    }

    public static unsafe string BytePtrToString(byte* bytes)
    {
        var result = Encoding.UTF8.GetString(bytes, NativeType.FindLength((bytes)));;
        return result;
    }
    
    public static unsafe List<string> BytePtrPtrToStringList(byte** bytePointerArray, int count)
    {
        List<string> result = new List<string>();
        for (int i = 0; i < count; i++)
        {
            int length = FindLength(bytePointerArray[i]);
            string str = Encoding.UTF8.GetString(bytePointerArray[i], length);
            result.Add(str);
        }
        return result;
    }
}