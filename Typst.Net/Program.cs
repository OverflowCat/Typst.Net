using System;
using System.Runtime.InteropServices;
using Typst.Net; // 引入生成的 Interop 命名空间

public static class Slicei8Converter
{
    public static (IntPtr data, ulong length) ConvertStringToSlicei8(string input)
    {
        // Convert the string to a byte array using UTF-8 encoding
        byte[] byteArray = System.Text.Encoding.UTF8.GetBytes(input);

        // Pin the byte array in memory
        GCHandle handle = GCHandle.Alloc(byteArray, GCHandleType.Pinned);

        // Get the pointer to the pinned object
        IntPtr pointer = handle.AddrOfPinnedObject();

        // Return the pointer and length as a tuple
        return (pointer, (ulong)byteArray.Length);
    }
}

class Program
{
    static void Main(string[] args)
    {
        // Vec2 input = new() { x = 5.0f, y = 3.0f };
        var s = "#box[hello world]\n";
        Slicei8 silde = Slicei8.FromString(s);
        TypstInput input2 = new() { content = silde };
        try
        {
            // Vec2 result = Interop.my_function(input);
            // Console.WriteLine($"Result: x = {result.x}, y = {result.y}");
            var rendered_svg = Interop.render_svg(input2);
            var result_string = rendered_svg.ToString();
            Console.WriteLine(result_string);
            // write to a file
            File.WriteAllText("output.svg", result_string);
        }
        catch (DllNotFoundException)
        {
            Console.WriteLine("Error: Native DLL 'library.dll' not found.");
        }
        catch (EntryPointNotFoundException)
        {
            Console.WriteLine("Error: Function 'my_function' not found in DLL.");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Unexpected error: {ex.Message}");
        }
    }
}