/// <summary>
/// Represents the exception thrown when the bad script is parsed.
/// </summary>
pub trait BadScriptException
{
    /// <summary>
    /// Initializes a new instance of the <see cref="BadScriptException"/> class.
    /// </summary>
    fn bad_script_exception();

    /// <summary>
    /// Initializes a new instance of the <see cref="BadScriptException"/> class with a specified error message.
    /// </summary>
    /// <param name="message">The message that describes the error.</param>
    fn bad_script_exception_str(message: &str);
}