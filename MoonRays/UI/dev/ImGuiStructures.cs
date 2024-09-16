using System.Runtime.InteropServices;

namespace MoonRays.UI.dev;

public static class ImGuiStructures
{
    public class ImVec2
    {
        public float x, y;
    }
    
    public class ImGuiKeyData
    {
        public bool Down;              // True for if key is down
        public float DownDuration;      // Duration the key has been down (<0.0f: not pressed, 0.0f: just pressed, >0.0f: time held)
        public float DownDurationPrev;  // Last frame duration the key has been down
        public float AnalogValue;       // 0.0f..1.0f for gamepad values
    }
    
    public unsafe class ImVector_ImWchar
    {
        public int Size;
        public int Capacity;
        public void* Data;
    }
    
    public struct ImGui_ImplVulkan_InitInfo
    {
        public IntPtr Instance;                      // VkInstance -> IntPtr
        public IntPtr PhysicalDevice;                // VkPhysicalDevice -> IntPtr
        public IntPtr Device;                        // VkDevice -> IntPtr
        public uint QueueFamily;                     // uint32_t -> uint
        public IntPtr Queue;                         // VkQueue -> IntPtr
        public IntPtr DescriptorPool;                // VkDescriptorPool -> IntPtr
        public IntPtr RenderPass;                    // VkRenderPass -> IntPtr
        public uint MinImageCount;                   // uint32_t -> uint
        public uint ImageCount;                      // uint32_t -> uint
        public uint MSAASamples;                     // VkSampleCountFlagBits -> uint (bitfield values can be mapped to uint)

        // Optional
        public IntPtr PipelineCache;                 // VkPipelineCache -> IntPtr
        public uint Subpass;                         // uint32_t -> uint

        // Optional Dynamic Rendering
        public bool UseDynamicRendering;             // bool -> bool

        // Optional Allocation, Debugging
        public IntPtr Allocator;                     // const VkAllocationCallbacks* -> IntPtr (can be null)
    
        // Delegate to represent the function pointer for CheckVkResultFn
        public delegate void CheckVkResultDelegate(int err); 
        public CheckVkResultDelegate CheckVkResultFn; // Function pointer -> Delegate

        public ulong MinAllocationSize;              // VkDeviceSize -> ulong (VkDeviceSize is typically an alias for uint64_t)
    }
    
    public struct ImGuiIO
    {
        public int ConfigFlags;
        public IntPtr BackendFlags;
        public IntPtr DisplaySize;
        public float DeltaTime;
        public float IniSavingRate;
        public IntPtr IniFilename;  // Use IntPtr for strings
        public IntPtr LogFilename;  // Use IntPtr for strings
        public IntPtr UserData;
        public IntPtr Fonts;        // Assuming ImFontAtlas is a pointer
        public float FontGlobalScale;
        public bool FontAllowUserScaling;
        public IntPtr FontDefault;  // Assuming ImFont is a pointer
        public IntPtr DisplayFramebufferScale;

        // Miscellaneous options
        public bool MouseDrawCursor;
        public bool ConfigMacOSXBehaviors;
        public bool ConfigNavSwapGamepadButtons;
        public bool ConfigInputTrickleEventQueue;
        public bool ConfigInputTextCursorBlink;
        public bool ConfigInputTextEnterKeepActive;
        public bool ConfigDragClickToInputText;
        public bool ConfigWindowsResizeFromEdges;
        public bool ConfigWindowsMoveFromTitleBarOnly;
        public float ConfigMemoryCompactTimer;

        // Inputs Behaviors
        public float MouseDoubleClickTime;
        public float MouseDoubleClickMaxDist;
        public float MouseDragThreshold;
        public float KeyRepeatDelay;
        public float KeyRepeatRate;

        // Debug options
        public bool ConfigDebugIsDebuggerPresent;
        public bool ConfigDebugBeginReturnValueOnce;
        public bool ConfigDebugBeginReturnValueLoop;
        public bool ConfigDebugIgnoreFocusLoss;
        public bool ConfigDebugIniSettings;

        // Platform Functions
        public IntPtr BackendPlatformName; // Use IntPtr for strings
        public IntPtr BackendRendererName; // Use IntPtr for strings
        public IntPtr BackendPlatformUserData;
        public IntPtr BackendRendererUserData;
        public IntPtr BackendLanguageUserData;

        // Input - Call before calling NewFrame()

        // Output - Updated by NewFrame() or EndFrame()/Render()
        public bool WantCaptureMouse;
        public bool WantCaptureKeyboard;
        public bool WantTextInput;
        public bool WantSetMousePos;
        public bool WantSaveIniSettings;
        public bool NavActive;
        public bool NavVisible;
        public float Framerate;
        public int MetricsRenderVertices;
        public int MetricsRenderIndices;
        public int MetricsRenderWindows;
        public int MetricsActiveWindows;
        public IntPtr MouseDelta;

        // [Internal] Dear ImGui will maintain those fields. Forward compatibility not guaranteed!
        public IntPtr Ctx;
        public IntPtr MousePos;
        public bool[] MouseDown;  // Ensure array size matches the actual C++ definition
        public float MouseWheel;
        public float MouseWheelH;
        public IntPtr MouseSource;
        public bool KeyCtrl;
        public bool KeyShift;
        public bool KeyAlt;
        public bool KeySuper;
        public IntPtr KeyMods;
        public IntPtr KeysData;
        public bool WantCaptureMouseUnlessPopupClose;
        public IntPtr MousePosPrev;
        public IntPtr MouseClickedPos;
        public double[] MouseClickedTime;
        public bool[] MouseClicked;
        public bool[] MouseDoubleClicked;
        public ushort[] MouseClickedCount;
        public ushort[] MouseClickedLastCount;
        public bool[] MouseReleased;
        public bool[] MouseDownOwned;
        public bool[] MouseDownOwnedUnlessPopupClose;
        public bool MouseWheelRequestAxisSwap;
        public bool MouseCtrlLeftAsRightClick;
        public float[] MouseDownDuration;
        public float[] MouseDownDurationPrev;
        public float[] MouseDragMaxDistanceSqr;
        public float PenPressure;
        public bool AppFocusLost;
        public bool AppAcceptingEvents;
        public sbyte BackendUsingLegacyKeyArrays;
        public bool BackendUsingLegacyNavInputArray;
        public ushort InputQueueSurrogate;
        public IntPtr InputQueueCharacters;  // Use IntPtr for pointer types

        // Legacy: before 1.87, we required backend to fill io.KeyMap[] (imgui->native map) during initialization and io.KeysDown[] (native indices) every frame.
        // This is still temporarily supported as a legacy feature. However, the new preferred scheme is for backend to call io.AddKeyEvent().
        //   Old (<1.87):  ImGui::IsKeyPressed(ImGui::GetIO().KeyMap[ImGuiKey_Space]) --> New (1.87+) ImGui::IsKeyPressed(ImGuiKey_Space)
        public int[] KeyMap;
        public bool[] KeysDown;
        public float[] NavInputs;
        //void* ImeWindowHandle;  // [Obsoleted in 1.87] Set ImGuiViewport::PlatformHandleRaw instead. Set this to your HWND to get automatic IME cursor positioning.
        public IntPtr ImeWindowHandle;
        // Legacy: before 1.91.1, clipboard functions were stored in ImGuiIO instead of ImGuiPlatformIO.
        // As this will affect all users of custom engines/backends, we are providing proper legacy redirection (will obsolete).
        public IntPtr GetClipboardTextFn;
        public IntPtr SetClipboardTextFn;
        public IntPtr ClipboardUserData;
    }
}