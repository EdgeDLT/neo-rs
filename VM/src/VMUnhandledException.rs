
use crate::StackItem::StackItem;

pub struct VMUnhandledException {
    ExceptionObject: StackItem,
};

impl VMUnhandledException {
    fn GetExceptionMessage(e: Box<StackItem>) -> Box<str>
    {
        let sb = new("An unhandled exception was thrown.");
        ByteString? s = e as ByteString;
        if (s is null && e is Array array && array.Count > 0)
            s = array[0] as ByteString;
        if (s != null)
        {
            sb.Append(' ');
            sb.Append(Encoding.UTF8.GetString(s.GetSpan()));
        }
        sb.ToString()
    }
}
// namespace Neo.VM
// {
//     /// <summary>
//     /// Represents an unhandled exception in the VM.
//     /// Thrown when there is an exception in the VM that is not caught by any script.
//     /// </summary>
//     public class VMUnhandledException : Exception
//     {
//         /// <summary>
//         /// The unhandled exception in the VM.
//         /// </summary>
//         public StackItem ExceptionObject { get; }
//
//         /// <summary>
//         /// Initializes a new instance of the <see cref="VMUnhandledException"/> class.
//         /// </summary>
//         /// <param name="ex">The unhandled exception in the VM.</param>
//         public VMUnhandledException(StackItem ex) : base(GetExceptionMessage(ex))
//         {
//             ExceptionObject = ex;
//         }
//
//         private static string GetExceptionMessage(StackItem e)
//         {
//             StringBuilder sb = new("An unhandled exception was thrown.");
//             ByteString? s = e as ByteString;
//             if (s is null && e is Array array && array.Count > 0)
//                 s = array[0] as ByteString;
//             if (s != null)
//             {
//                 sb.Append(' ');
//                 sb.Append(Encoding.UTF8.GetString(s.GetSpan()));
//             }
//             return sb.ToString();
//         }
//     }
// }
