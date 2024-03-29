namespace Qynit.PulseGen.Schedules;
public sealed class SetPhaseElement(int channelId, double phase) : ScheduleElement
{
    private HashSet<int>? _channels;
    public override IReadOnlySet<int> Channels => _channels ??= [ChannelId];
    public int ChannelId { get; } = channelId;
    public double Phase { get; } = phase;

    protected override double ArrangeOverride(double time, double finalDuration)
    {
        return 0;
    }

    protected override double MeasureOverride(double maxDuration)
    {
        return 0;
    }

    protected override void RenderOverride(double time, PhaseTrackingTransform phaseTrackingTransform)
    {
        phaseTrackingTransform.SetPhase(ChannelId, Phase, time);
    }
}
