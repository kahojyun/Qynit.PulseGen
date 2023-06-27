﻿using System.Numerics;

namespace Qynit.Pulsewave;
public class TrianglePulseShape : IPulseShape
{
    public IqPair<T> SampleAt<T>(T x) where T : unmanaged, IFloatingPointIeee754<T>
    {
        var half = T.CreateChecked(0.5);
        var i = (x >= -half && x <= half) ? (T.One - T.CreateChecked(2) * T.Abs(x)) : T.Zero;
        return i;
    }
}
