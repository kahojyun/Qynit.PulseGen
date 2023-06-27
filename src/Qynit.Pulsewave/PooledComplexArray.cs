﻿using System.Buffers;
using System.Runtime.CompilerServices;

using CommunityToolkit.Diagnostics;

namespace Qynit.Pulsewave;
public sealed class PooledComplexArray<T> : IDisposable
    where T : unmanaged
{
    public int Length { get; }
    public bool IsEmpty => Length == 0;
    public Span<T> DataI
    {
        get
        {
            if (_disposed)
            {
                ThrowHelper.ThrowObjectDisposedException(nameof(PooledComplexArray<T>));
            }
            return _dataI.AsSpan(0, Length);
        }
    }
    public Span<T> DataQ
    {
        get
        {
            if (_disposed)
            {
                ThrowHelper.ThrowObjectDisposedException(nameof(PooledComplexArray<T>));
            }
            return _dataQ.AsSpan(0, Length);
        }
    }
    private readonly T[] _dataI;
    private readonly T[] _dataQ;
    private bool _disposed;

    public PooledComplexArray(int length, bool clear)
    {
        Length = length;
        _dataI = ArrayPool<T>.Shared.Rent(length);
        _dataQ = ArrayPool<T>.Shared.Rent(length);
        if (clear)
        {
            Clear();
        }
    }
    public PooledComplexArray(ComplexArrayReadOnlySpan<T> source) : this(source.Length, false)
    {
        source.DataI.CopyTo(_dataI);
        source.DataQ.CopyTo(_dataQ);
    }

    public PooledComplexArray<T> Copy()
    {
        return new PooledComplexArray<T>(this);
    }
    public void CopyTo(ComplexArraySpan<T> destination)
    {
        ((ComplexArrayReadOnlySpan<T>)this).CopyTo(destination);
    }
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public ComplexArraySpan<T> Slice(int start)
    {
        return new ComplexArraySpan<T>(DataI[start..], DataQ[start..]);
    }
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public ComplexArraySpan<T> Slice(int start, int length)
    {
        return new ComplexArraySpan<T>(DataI.Slice(start, length), DataQ.Slice(start, length));
    }
    public void Clear()
    {
        DataI.Clear();
        DataQ.Clear();
    }
    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }
        _disposed = true;
        ArrayPool<T>.Shared.Return(_dataI, false);
        ArrayPool<T>.Shared.Return(_dataQ, false);
    }
}
