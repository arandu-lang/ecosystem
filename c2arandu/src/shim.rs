//! Shims C gerados para construções que não têm equivalente direto em Arandu:
//! chamadas variádicas, definições variádicas (va_list), `va_arg` e chamadas
//! indiretas por ponteiro de função.
//!
//! Cada shim é uma função C não-variádica com assinatura concreta. O lado
//! Arandu enxerga apenas uma declaração `extern "C"` comum; o lado C faz a
//! ponte com a ABI variádica ou com o ponteiro de função. A assinatura é
//! derivada dos tipos canônicos do Clang, nunca do texto da chamada.

use std::cell::RefCell;
use std::collections::BTreeMap;

use clang_sys::*;

/// Classe ABI de um valor que atravessa um shim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AbiClass {
    Void,
    I32,
    U32,
    I64,
    U64,
    F64,
    Ptr,
}

impl AbiClass {
    pub fn code(self) -> char {
        match self {
            AbiClass::Void => 'v',
            AbiClass::I32 => 'i',
            AbiClass::U32 => 'u',
            AbiClass::I64 => 'l',
            AbiClass::U64 => 'm',
            AbiClass::F64 => 'd',
            AbiClass::Ptr => 'p',
        }
    }

    pub fn c_type(self) -> &'static str {
        match self {
            AbiClass::Void => "void",
            AbiClass::I32 => "int",
            AbiClass::U32 => "unsigned int",
            AbiClass::I64 => "long long",
            AbiClass::U64 => "unsigned long long",
            AbiClass::F64 => "double",
            AbiClass::Ptr => "void *",
        }
    }

    /// Tipo Arandu usado no parâmetro do shim.
    pub fn arandu_param(self) -> &'static str {
        match self {
            AbiClass::Void => "void",
            AbiClass::I32 => "int",
            AbiClass::U32 => "uint",
            AbiClass::I64 => "i64",
            AbiClass::U64 => "u64",
            AbiClass::F64 => "f64",
            AbiClass::Ptr => "ptr[u8]?",
        }
    }

    /// Tipo alvo do cast aplicado ao argumento Arandu antes da chamada.
    pub fn arandu_cast(self) -> &'static str {
        match self {
            AbiClass::Ptr => "ptr[u8]",
            other => other.arandu_param(),
        }
    }
}

/// Classifica um tipo C (já com promoções aplicadas pelo chamador quando
/// necessário). Retorna `None` para agregados por valor, que não são
/// suportados por shims.
pub unsafe fn classify(ty: CXType) -> Option<AbiClass> {
    let canon = clang_getCanonicalType(ty);
    let size = clang_Type_getSizeOf(canon);
    match canon.kind {
        CXType_Void => Some(AbiClass::Void),
        CXType_Pointer | CXType_BlockPointer | CXType_ConstantArray | CXType_IncompleteArray => {
            Some(AbiClass::Ptr)
        }
        CXType_FunctionProto | CXType_FunctionNoProto => Some(AbiClass::Ptr),
        CXType_Float | CXType_Double => Some(AbiClass::F64),
        CXType_Bool | CXType_Char_S | CXType_SChar | CXType_Short | CXType_Int | CXType_Enum => {
            if size > 4 { Some(AbiClass::I64) } else { Some(AbiClass::I32) }
        }
        CXType_Char_U | CXType_UChar | CXType_UShort => Some(AbiClass::I32),
        CXType_UInt => Some(AbiClass::U32),
        CXType_Long | CXType_LongLong => Some(AbiClass::I64),
        CXType_ULong | CXType_ULongLong => Some(AbiClass::U64),
        // `va_list` decai para ponteiro em parâmetros.
        CXType_Elaborated | CXType_Typedef => classify(canon),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct ShimDef {
    /// Nome do símbolo do shim (mesmo nome em Arandu e em C).
    pub name: String,
    /// Parâmetros Arandu (tipos) na ordem da chamada.
    pub params: Vec<AbiClass>,
    /// Tipo de retorno Arandu (exato, para preservar o tipo do resultado).
    pub arandu_ret: String,
    /// Corpo C completo da definição do shim.
    pub c_def: String,
}

#[derive(Default)]
struct Registry {
    shims: BTreeMap<String, ShimDef>,
    /// Funções variádicas definidas no próprio C transpilado: nome C →
    /// (classes fixas, retorno).
    variadic_defs: BTreeMap<String, (Vec<AbiClass>, AbiClass)>,
}

thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

fn sig_code(classes: &[AbiClass]) -> String {
    classes.iter().map(|c| c.code()).collect()
}

fn c_params(classes: &[AbiClass]) -> (String, String) {
    let decl = classes
        .iter()
        .enumerate()
        .map(|(i, c)| format!("{} a{i}", c.c_type()))
        .collect::<Vec<_>>()
        .join(", ");
    let args = (0..classes.len()).map(|i| format!("a{i}")).collect::<Vec<_>>().join(", ");
    (if decl.is_empty() { "void".to_string() } else { decl }, args)
}

fn ret_stmt(ret: AbiClass, call: &str) -> String {
    if ret == AbiClass::Void {
        format!("{call};")
    } else {
        format!("return {call};")
    }
}

/// Shim para uma chamada direta a uma função variádica `callee` (nome C).
pub fn variadic_call(
    callee: &str,
    fixed: &[AbiClass],
    extra: &[AbiClass],
    ret: AbiClass,
    arandu_ret: &str,
) -> String {
    let mut all = fixed.to_vec();
    all.extend_from_slice(extra);
    let name = format!("aranduVa_{}_{}_{}", callee, ret.code(), sig_code(&all));
    REGISTRY.with(|r| {
        let mut r = r.borrow_mut();
        if r.shims.contains_key(&name) {
            return;
        }
        let (decl, args) = c_params(&all);
        let fixed_proto = fixed.iter().map(|c| c.c_type()).collect::<Vec<_>>().join(", ");
        let fn_ty = format!("{} (*)({}{}...)", ret.c_type(), fixed_proto, if fixed.is_empty() { "" } else { ", " });
        let c_def = format!(
            "extern void {callee}(void);\n{} {name}({decl}) {{\n    {}\n}}\n",
            ret.c_type(),
            ret_stmt(ret, &format!("(({fn_ty})(void (*)(void)){callee})({args})"))
        );
        r.shims.insert(
            name.clone(),
            ShimDef { name: name.clone(), params: all, arandu_ret: arandu_ret.to_string(), c_def },
        );
    });
    name
}

/// Shim para chamada indireta através de um ponteiro de função.
pub fn indirect_call(params: &[AbiClass], ret: AbiClass, arandu_ret: &str) -> String {
    let name = format!("aranduInd_{}_{}", ret.code(), sig_code(params));
    REGISTRY.with(|r| {
        let mut r = r.borrow_mut();
        if r.shims.contains_key(&name) {
            return;
        }
        let mut all = vec![AbiClass::Ptr];
        all.extend_from_slice(params);
        let (decl, _) = c_params(&all);
        let args = (1..all.len()).map(|i| format!("a{i}")).collect::<Vec<_>>().join(", ");
        let proto = if params.is_empty() {
            "void".to_string()
        } else {
            params.iter().map(|c| c.c_type()).collect::<Vec<_>>().join(", ")
        };
        let c_def = format!(
            "{} {name}({decl}) {{\n    {}\n}}\n",
            ret.c_type(),
            ret_stmt(ret, &format!("(({} (*)({proto}))a0)({args})", ret.c_type()))
        );
        r.shims.insert(
            name.clone(),
            ShimDef { name: name.clone(), params: all, arandu_ret: arandu_ret.to_string(), c_def },
        );
    });
    name
}

/// Shim para `va_arg(ap, T)`. Em Arandu, `VaList` é sempre um ponteiro para
/// um objeto `va_list` vivo no frame do wrapper C.
pub fn va_arg(class: AbiClass, arandu_ret: &str) -> String {
    let name = format!("aranduVaArg_{}", class.code());
    REGISTRY.with(|r| {
        let mut r = r.borrow_mut();
        if r.shims.contains_key(&name) {
            return;
        }
        let c_def = format!(
            "{} {name}(void *ap) {{\n    return va_arg(*(va_list *)ap, {});\n}}\n",
            class.c_type(),
            class.c_type()
        );
        r.shims.insert(
            name.clone(),
            ShimDef { name: name.clone(), params: vec![AbiClass::Ptr], arandu_ret: arandu_ret.to_string(), c_def },
        );
    });
    name
}

/// Registra uma definição variádica do programa: o corpo vira a função
/// Arandu exportada `aranduV_<nome>` (com `VaList` final) e o símbolo C
/// original passa a ser um wrapper que faz `va_start`/`va_end`.
pub fn variadic_definition(c_name: &str, fixed: &[AbiClass], ret: AbiClass) {
    REGISTRY.with(|r| {
        r.borrow_mut().variadic_defs.insert(c_name.to_string(), (fixed.to_vec(), ret));
    });
}

pub fn exported_variadic_body_name(c_name: &str) -> String {
    format!("aranduV_{c_name}")
}

pub fn is_variadic_definition(c_name: &str) -> bool {
    REGISTRY.with(|r| r.borrow().variadic_defs.contains_key(c_name))
}

pub fn take_shims() -> Vec<ShimDef> {
    REGISTRY.with(|r| r.borrow().shims.values().cloned().collect())
}

/// Gera o arquivo C completo de shims.
pub fn render_c_file() -> String {
    REGISTRY.with(|r| {
        let r = r.borrow();
        let mut out = String::new();
        out.push_str("/* Gerado automaticamente pelo c2arandu. Não editar. */\n");
        out.push_str("/* Compile com -fno-builtin: os protótipos abaixo são genéricos. */\n");
        out.push_str("#include <stdarg.h>\n\n");
        for (c_name, (fixed, ret)) in &r.variadic_defs {
            let Some(last) = fixed.len().checked_sub(1) else { continue };
            let mut body_params = fixed.clone();
            body_params.push(AbiClass::Ptr);
            let body_proto = body_params.iter().map(|c| c.c_type()).collect::<Vec<_>>().join(", ");
            let body_name = exported_variadic_body_name(c_name);
            let (decl, args) = c_params(fixed);
            out.push_str(&format!("{} {body_name}({body_proto});\n", ret.c_type()));
            let call = format!("{body_name}({args}, (void *)&ap)");
            let body = if *ret == AbiClass::Void {
                format!("    {call};\n    va_end(ap);\n")
            } else {
                format!("    {} r = {call};\n    va_end(ap);\n    return r;\n", ret.c_type())
            };
            out.push_str(&format!(
                "{} {c_name}({decl}, ...) {{\n    va_list ap;\n    va_start(ap, a{last});\n{body}}}\n\n",
                ret.c_type()
            ));
        }
        for shim in r.shims.values() {
            out.push_str(&shim.c_def);
            out.push('\n');
        }
        out
    })
}
