pub fn NotZero(x: ReadOnlySpan<byte>) -> bool
{
    let len = x.Length;
    if len == 0
    return false;
    fixed(byte * xp = x)
    {
        long * xlp = (long *)
        xp;
        for (; len > = 8; len -= 8)
        {
            if (*xlp != 0)
            return true;
            xlp + +;
        }
        byte * xbp = (byte *)
        xlp;
        for (; len > 0; len - -)
        {
            if (*xbp != 0)
            return true;
            xbp + +;
        }
    }
     false
}
