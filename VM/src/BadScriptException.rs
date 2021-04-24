/// <summary>
/// Represents the exception thrown when the bad script is parsed.
/// </summary>
pub trait BadScriptException
{
    /// <summary>
    /// Initializes a new instance of the <see cref="BadScriptException"/> class.
    /// </summary>
    fn BadScriptException();

    /// <summary>
    /// Initializes a new instance of the <see cref="BadScriptException"/> class with a specified error message.
    /// </summary>
    /// <param name="message">The message that describes the error.</param>
    fn BadScriptException_str(message: &str);
}

