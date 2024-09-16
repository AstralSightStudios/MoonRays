using System.Runtime.InteropServices;

namespace MoonRays.UI.dev;

public static unsafe class ImGuiBinding
{
    // Change this when compile or run with imgui on other platforms.
    const string LibName = "Native/libcimgui-win64.dll";

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void hello();
    
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern byte* ImGui_GetVersion();
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void* ImGui_CreateContext();
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void* ImGui_GetIO();
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern bool cImGui_ImplSDL2_InitForVulkan(void* sdlWindow);
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern bool cImGui_ImplVulkan_Init(ref ImGuiStructures.ImGui_ImplVulkan_InitInfo info);
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern bool cImGui_ImplVulkan_CreateFontsTexture();
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern bool cImGui_ImplSDL2_ProcessEvent(void* sdlEvent);
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void cImGui_ImplVulkan_NewFrame();
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void cImGui_ImplSDL2_NewFrame();
    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void cImGui_ImplVulkan_RenderDrawData(void* drawData, IntPtr vkCommandBuffer);
}