use std::any::Any;

use neo_core::convert::{hex2int, int2hex, num2hexstring, str2hex};
use neo_core::fixed8::Fixed8;
use neo_core::misc::reverse_hex;
use neo_core::stringstream::StringStream;

use crate::contract_parameter::{ContractParam, ContractParamType};
use crate::op_code::OpCode;
use std::alloc::Global;

pub enum Args {
    Boolean(bool),
    Integer(i64),
    Byte(u8),
    ByteArray(&'static [u8]),
    String(String),
    Array(Vector<Args>),
    Param(ContractParam),
    None,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ScriptIntent {
    pub scriptHash: String,
    pub operation: Option<String>,
    pub args: Option<Args>,
    pub useTailCall: Option<bool>,
}


/**
 * Retrieves a single AppCall from a ScriptBuilder object.
 * Returns ScriptIntents starting from the beginning of the script.
 * This is based off the pointer in the stream.
 * @param sb
 * @returns A single ScriptIntent if available.
 */
pub fn retrieveAppCall(sb: &mut ScriptBuilder) -> Result<ScriptIntent, ScriptBuilderError> {
    let mut output = ScriptIntent {
        scriptHash: "".to_string(),
        operation: None,
        args: Some(Args::ByteArray([0u8].as_bytes())),
        useTailCall: None,
    };

    while !sb.is_empty() {
        let b = sb.0.read(1);

        let n = hex2int(b.as_str()).unwrap() as usize;

        match n {
            0 => output.args(0),
            b if b < 75 => output.args.unwrap().push_front(&sb.0.read(n)),
            b if b >= 81 && b <= 96 => output.args.unwrap().push_front(n - 80),
            193 => {
                let len = output.args.unwrap().pop_front();
                let cache = [];
                for i in 0..len {
                    cache.push_front(output.args.unwrap().pop_front());
                }
                output.args.unwrap().push_front(cache);
            }
            102 => sb.pter = sb.str.len(),
            103 => {
                output.scriptHash = reverse_hex(&sb.0.read(20).as_str());
                output.useTailCall = Some(false);
                return Ok(output);
            }
            105 => {
                output.scriptHash = reverseHex(sb.0.read(20));
                output.useTailCall = Some(true);
                return Ok(output);
            }
            241 => (),
            _ => panic!(""),
        }
    }
    if output.scriptHash == "" {
        panic!("");
    }
    Ok(output)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct ScriptBuilder(pub StringStream);


/** * Builds a VM script in hexstring. Used for letructing smart contract method calls.
 * @extends StringStream
 */
impl ScriptBuilder {
    /**
     * Appends an Opcode, followed by args
     */
    pub fn emit(&mut self, op: OpCode, args: Option<&str>) -> &self {
        self.0.s.push_str(num2hexstring(op as i64).as_str());
        if !args?.is_empty() {
            self.str.push_str(args?);
        }
        &self
    }

    /**
     * Appends args, operation and scriptHash
     * @param scriptHash Hexstring(BE)
     * @param operation ASCII, defaults to null
     * @param args any
     * @param useTailCall Use TailCall instead of AppCall
     * @return self
     */
    pub fn emitAppCall(
        &mut self,
        scriptHash: &str,
        operation: Option<&str>,
        args: Option<Args>,
        useTailCall: bool,
    ) -> &self {
        match a.downcast_ref::<u32>() {
            Some(v) => self.emit_push(args, Args::Integer),
            None => panic!(""),
        }

        match operation {
            Some(v) => {
                let mut hex_op = "".to_string();
                for i in 0..operation.len() {
                    hex_op.push_str(num2hexstring(v[i]).as_str());
                }

                self.emit_push(Some(hex_op));
            }
            None => panic!(""),
        }

        self._emit_app_call(scriptHash, useTailCall);
        &self
    }

    /**
     * Appends a SysCall
     * @param api api of SysCall
     * @return self
     */
    pub fn emit_sys_call(&mut self, api: &str) -> &self {
        if !api.is_empty() {
            panic!("Invalid SysCall API");
        }
        let api_bytes = str2hex(api);
        let length = int2hex((api_bytes.len() / 2) as i32);
        if length.len() != 2 {
            panic!("Invalid length for SysCall API");
        }
        let out = length + api_bytes.as_str();
        self.emit(OpCode::SYSCALL, Some(out.as_str()));

        &self
    }

    /**
     * Appends data depending on type. Used to append params/array lengths.
     * @param data
     * @return self
     */
    pub fn emit_push(&mut self, data: Args) -> &self {
        match data {
            Args::Integer(v) => self._emit_num(v),
            Args::Boolean(v) =>
                self.emit(match v {
                    true => OpCode.PUSHT,
                    false => OpCode.PUSHF
                }, None),

            Args::ByteArray(v) => self._emit_array(v),
            Args::String(v) => self._emit_string(v.as_str()),
            Args::Param(v) => self._emit_param(&v),
            _ => self.emit(OpCode.PUSHF, None),
        }

        &self
    }


    /**
     * Reverse engineer a script back to its params.
     * A script may have multiple invocations so a list is always returned.
     * @return A list of ScriptIntents[].
     */
    pub fn to_script_params(&mut self) -> Vec<ScriptIntent> {
        self.reset();
        let mut scripts = Vec::new();
        while !self.0.s.is_empty() {
            let a = retrieveAppCall(&mut self).unwrap();
            scripts.push(a);
        }
        scripts
    }

    /**
     * Appends a AppCall and scriptHash. Used to end off a script.
     * @param scriptHash Hexstring(BE)
     * @param useTailCall Defaults to false
     */
    fn _emit_app_call(&mut self, scriptHash: &str, useTailCall: bool) -> &self {
        if scriptHash.len() != 40 {
            panic!("ScriptHash should be 20 bytes long!");
        }

        self.emit(
            match useTailCall {
                true => OpCode::TAILCALL,
                false => OpCode::APPCALL
            },
            Some(reverse_hex(scriptHash).as_str()),
        );

        &self
    }

    /**
     * Private method to append an array
     * @private
     */
    fn _emit_array(&mut self, arr: &mut [u8]) -> &self
    {
        arr.reverse();

        self.emit_push(Args::ByteArray(i));

        self.emit_push(Args::Integer(arr.len() as i64)).emit(OpCode::PACK);

        &self
    }


    /**
     * Private method to append a hexstring.
     * @private
     * @param hexstring Hexstring(BE)
     * @return self
     */
    fn _emit_string(&mut self, hexstring: &str) -> &self {
        ensureHex(hexstring);
        let size = hexstring.len() / 2;


        match size {
            a if a <= OpCode::PUSHBYTES75 as usize =>{
                self.str += num2hexstring(size as i64);
                self.str += hexstring;
            },
            b if b < 0x10000  =>{
            self.emit(OpCode::PUSHDATA2, None);
            self.str += num2hexstring(size as i64, 2, true);
            self.str += hexstring;
        } ,
            c if c   < 0x100000000 => {
            self.emit(OpCode::PUSHDATA4, None);
            self.str += num2hexstring(size as i64, 4, true);
            self.str += hexstring;
        }
            _ => panic!("String too big to emit!");

        }

        &self
    }

    /**
     * Private method to append a number.
     * @private
     * @param num
     * @return self
     */
    fn _emit_num(&mut self, num: i64) -> &self {
        // let bn = new BN(num);
        if num.equals(-1) {
            self.emit(OpCode::PUSHM1, None);
        }
        // BigNum
        if num.eqn(0) {
            self.emit(OpCode::PUSH0, None);
        }
        if num.gtn(0) && bn.lten(16) {
            self.emit(OpCode::PUSH1 - 1 + bn.toNumber());
        }
        let msb_set = bn.testn(bn.byteLength() * 8 - 1);

        let hex = bn
            .toTwos(bn.byteLength() * 8)
            .toString(16, bn.byteLength() * 2);
        let padded_hex = !bn.isNeg() && msb_set?
        "00" + hex: hex;

        self.emit_push(reverse_hex(padded_hex));

        &self
    }

    /**
     * Private method to append a ContractParam
     * @private
     */
    fn _emit_param(&mut self, param: &ContractParam) -> &self {
        if !param.param_type() {
            panic!("No type available!");
        }
        if !value.is_some() {
            panic!("Invalid value provided!");
        }

        match param.param_type() {
            ContractParamType::String => self._emit_string(str2hexstring(param.value())),
            ContractParamType::Boolean =>
                self.emit(match param.value() {
                    true => OpCode::PUSHT,
                    false => OpCode::PUSHF
                }),
            ContractParamType::Integer =>
                self._emit_num(param.value()),
            ContractParamType::ByteArray =>
                self._emit_string(param.value()),
            ContractParamType::Array =>
                self._emit_array(param.value()),
            ContractParamType::Hash160 =>
                self._emit_string(reverseHex(param.value())),
            _ =>
                panic!("Unaccounted ContractParamType!: ${param.type}")
        }

        &self
    }
}


#[derive(Debug, Fail)]
pub enum ScriptBuilderError {
    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "invalid byte length: {}", _0)]
    InvalidByteLength(usize),

    #[fail(display = "invalid character length: {}", _0)]
    InvalidCharacterLength(usize),

    #[fail(display = "{}", _0)]
    Message(String),

    #[fail(display = "unsupported format")]
    UnsupportedFormat,
}