


    /// <summary>
    /// Represents an interface used to interoperate with the outside of the the VM.
    /// </summary>
    [DebuggerDisplay("Type={GetType().Name}, Value={_object}")]
    public class InteropInterface : StackItem
    {
        private readonly object _object;

        public override StackItemType Type => StackItemType.InteropInterface;

        /// <summary>
        /// Create an interoperability interface that wraps the specified <see cref="object"/>.
        /// </summary>
        /// <param name="value">The wrapped <see cref="object"/>.</param>
        public InteropInterface(object value)
        {
            _object = value ?? throw new ArgumentNullException(nameof(value));
        }

        public override bool Equals(StackItem? other)
        {
            if (ReferenceEquals(this, other)) return true;
            if (other is InteropInterface i) return _object.Equals(i._object);
            return false;
        }

        public override bool GetBoolean()
        {
            return true;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(_object);
        }

        public override T GetInterface<T>()
        {
            if (_object is T t) return t;
            throw new InvalidCastException($"The item can't be casted to type {typeof(T)}");
        }
    }