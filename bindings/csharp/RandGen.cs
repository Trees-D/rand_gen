using System;
using System.Runtime.InteropServices;

namespace RandGen
{
    public static class RandGenNative
    {
        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr spawn_rand_gen(ulong in_seed);

        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern void free_rand_gen(IntPtr rng);

        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern ulong next_u64(IntPtr rng);

        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern uint next_u32(IntPtr rng);

        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern float next_f32(IntPtr rng);

        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern double next_f64(IntPtr rng);

        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern void next_u64n(IntPtr rng, uint n, [Out] ulong[] out_values);

        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern void next_u32n(IntPtr rng, uint n, [Out] uint[] out_values);

        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern void next_f32n(IntPtr rng, uint n, [Out] float[] out_values);

        [DllImport("addons/Rand/rand_gen.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern void next_f64n(IntPtr rng, uint n, [Out] double[] out_values);
    }
}
