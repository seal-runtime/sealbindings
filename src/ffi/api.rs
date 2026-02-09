#![allow(nonstandard_style)]

use crate::ffi::types::*;
use std::os::raw::{c_char, c_double, c_float, c_int, c_void};


#[repr(C)]
pub struct LuauApi {
    //
    // State manipulation
    //
    pub lua_newstate: unsafe extern "C-unwind" fn(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State,
    pub lua_close: unsafe extern "C-unwind" fn(state: *mut lua_State),
    pub lua_newthread: unsafe extern "C-unwind" fn(state: *mut lua_State) -> *mut lua_State,
    pub lua_mainthread: unsafe extern "C-unwind" fn(state: *mut lua_State) -> *mut lua_State,
    pub lua_resetthread: unsafe extern "C-unwind" fn(state: *mut lua_State),
    pub lua_isthreadreset: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,

    //
    // Basic stack manipulation
    //
    pub lua_absindex: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_gettop: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_settop: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_pushvalue: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_remove: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_insert: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_replace: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_checkstack: unsafe extern "C-unwind" fn(state: *mut lua_State, sz: c_int) -> c_int,
    pub lua_rawcheckstack: unsafe extern "C-unwind" fn(state: *mut lua_State, sz: c_int),

    pub lua_xmove: unsafe extern "C-unwind" fn(from: *mut lua_State, to: *mut lua_State, n: c_int),
    pub lua_xpush: unsafe extern "C-unwind" fn(from: *mut lua_State, to: *mut lua_State, idx: c_int),

    //
    // Access functions (stack -> C)
    //
    pub lua_isnumber: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_isstring: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_iscfunction: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_isLfunction: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_isuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_type: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_typename: unsafe extern "C-unwind" fn(state: *mut lua_State, tp: c_int) -> *const c_char,

    pub lua_equal: unsafe extern "C-unwind" fn(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int,
    pub lua_rawequal: unsafe extern "C-unwind" fn(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int,
    pub lua_lessthan: unsafe extern "C-unwind" fn(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int,

    pub lua_tonumberx: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        isnum: *mut c_int,
    ) -> lua_Number,
    pub lua_tointegerx_: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        isnum: *mut c_int,
    ) -> c_int,
    pub lua_tounsignedx: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        isnum: *mut c_int,
    ) -> lua_Unsigned,
    pub lua_tovector: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *const c_float,
    pub lua_toboolean: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_tolstring: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        len: *mut usize,
    ) -> *const c_char,
    pub lua_tostringatom: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        atom: *mut c_int,
    ) -> *const c_char,
    pub lua_namecallatom: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        atom: *mut c_int,
    ) -> *const c_char,
    pub lua_objlen: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> usize,
    pub lua_tocfunction: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
    ) -> Option<lua_CFunction>,
    pub lua_tolightuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *mut c_void,
    pub lua_tolightuserdatatagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        tag: c_int,
    ) -> *mut c_void,
    pub lua_touserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *mut c_void,
    pub lua_touserdatatagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        tag: c_int,
    ) -> *mut c_void,
    pub lua_userdatatag: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_lightuserdatatag: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_tothread: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *mut lua_State,
    pub lua_tobuffer: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        len: *mut usize,
    ) -> *mut c_void,
    pub lua_topointer: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> *const c_void,

    //
    // Push functions (C -> stack)
    //
    pub lua_pushnil: unsafe extern "C-unwind" fn(state: *mut lua_State),
    pub lua_pushnumber: unsafe extern "C-unwind" fn(state: *mut lua_State, n: lua_Number),
    pub lua_pushinteger_: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int),
    pub lua_pushunsigned: unsafe extern "C-unwind" fn(state: *mut lua_State, n: lua_Unsigned),
    pub lua_pushvector: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        x: c_float,
        y: c_float,
        z: c_float,
    ),
    pub lua_pushlstring_: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        s: *const c_char,
        l: usize,
    ),
    pub lua_pushstring_: unsafe extern "C-unwind" fn(state: *mut lua_State, s: *const c_char),
    // pub lua_pushfstring: unsafe extern "C-unwind" fn(
    //     state: *mut lua_State,
    //     fmt: *const c_char,
    //     ...
    // ) -> *const c_char,
    pub lua_pushcclosurek: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        debugname: *const c_char,
        nup: c_int,
        cont: Option<lua_Continuation>,
    ),
    pub lua_pushboolean: unsafe extern "C-unwind" fn(state: *mut lua_State, b: c_int),
    pub lua_pushthread: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,

    pub lua_pushlightuserdatatagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        p: *mut c_void,
        tag: c_int,
    ),
    pub lua_newuserdatatagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        sz: usize,
        tag: c_int,
    ) -> *mut c_void,
    pub lua_newuserdatataggedwithmetatable: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        sz: usize,
        tag: c_int,
    ) -> *mut c_void,
    pub lua_newuserdatadtor: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        sz: usize,
        dtor: lua_Destructor,
    ) -> *mut c_void,

    pub lua_newbuffer: unsafe extern "C-unwind" fn(state: *mut lua_State, sz: usize) -> *mut c_void,

    //
    // Get functions (Lua -> stack)
    //
    pub lua_gettable: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_getfield: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        k: *const c_char,
    ) -> c_int,
    pub lua_rawgetfield: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        k: *const c_char,
    ) -> c_int,
    pub lua_rawget: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_rawgeti_: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, n: c_int) -> c_int,
    pub lua_rawgetptagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        p: *const c_void,
        tag: c_int,
    ) -> c_int,
    pub lua_createtable: unsafe extern "C-unwind" fn(state: *mut lua_State, narr: c_int, nrec: c_int),

    pub lua_setreadonly: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, enabled: c_int),
    pub lua_getreadonly: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_setsafeenv: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, enabled: c_int),

    pub lua_getmetatable: unsafe extern "C-unwind" fn(state: *mut lua_State, objindex: c_int) -> c_int,
    pub lua_getfenv: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),

    //
    // Set functions (stack -> Lua)
    //
    pub lua_settable: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_setfield: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        k: *const c_char,
    ),
    pub lua_rawset: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_rawseti_: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, n: c_int),
    pub lua_rawsetptagged: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        idx: c_int,
        p: *const c_void,
        tag: c_int,
    ),
    pub lua_setmetatable: unsafe extern "C-unwind" fn(state: *mut lua_State, objindex: c_int) -> c_int,
    pub lua_setfenv: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,

    //
    // load / call
    //
    pub luau_load: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        chunkname: *const c_char,
        data: *const c_char,
        size: usize,
        env: c_int,
    ) -> c_int,
    pub lua_call: unsafe extern "C-unwind" fn(state: *mut lua_State, nargs: c_int, nresults: c_int),
    pub lua_pcall: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
        errfunc: c_int,
    ) -> c_int,
    pub lua_cpcall: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        ud: *mut c_void,
    ) -> c_int,

    //
    // Coroutine
    //
    pub lua_yield: unsafe extern "C-unwind" fn(state: *mut lua_State, nresults: c_int) -> c_int,
    pub lua_break: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_resume_: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        from: *mut lua_State,
        narg: c_int,
    ) -> c_int,
    pub lua_resumeerror: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        from: *mut lua_State,
    ) -> c_int,
    pub lua_status: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_isyieldable: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_getthreaddata: unsafe extern "C-unwind" fn(state: *mut lua_State) -> *mut c_void,
    pub lua_setthreaddata: unsafe extern "C-unwind" fn(state: *mut lua_State, data: *mut c_void),

    //
    // GC
    //
    pub lua_gc: unsafe extern "C-unwind" fn(state: *mut lua_State, what: c_int, data: c_int) -> c_int,
    pub lua_gcstatename: unsafe extern "C-unwind" fn(state: c_int) -> *const c_char,
    pub lua_gcallocationrate: unsafe extern "C-unwind" fn(state: *mut lua_State) -> i64,

    //
    // Memory stats
    //
    pub lua_setmemcat: unsafe extern "C-unwind" fn(state: *mut lua_State, category: c_int),
    pub lua_totalbytes: unsafe extern "C-unwind" fn(state: *mut lua_State, category: c_int) -> usize,

    //
    // Misc
    //
    pub lua_error: unsafe extern "C-unwind" fn(state: *mut lua_State) -> !,
    pub lua_next: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_rawiter: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, iter: c_int) -> c_int,
    pub lua_concat: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int),
    pub lua_clock: unsafe extern "C-unwind" fn() -> c_double,
    pub lua_setuserdatatag: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int, tag: c_int),
    pub lua_setuserdatadtor: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        tag: c_int,
        dtor: Option<lua_Destructor>,
    ),
    pub lua_getuserdatadtor: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        tag: c_int,
    ) -> Option<lua_Destructor>,
    pub lua_setuserdatametatable: unsafe extern "C-unwind" fn(state: *mut lua_State, tag: c_int),
    pub lua_getuserdatametatable: unsafe extern "C-unwind" fn(state: *mut lua_State, tag: c_int),
    pub lua_setlightuserdataname: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        tag: c_int,
        name: *const c_char,
    ),
    pub lua_getlightuserdataname: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        tag: c_int,
    ) -> *const c_char,
    pub lua_clonefunction: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_cleartable: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int),
    pub lua_getallocf: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        ud: *mut *mut c_void,
    ) -> lua_Alloc,

    //
    // Reference system
    //
    pub lua_ref: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_unref: unsafe extern "C-unwind" fn(state: *mut lua_State, r#ref: c_int),

    //
    // Debug API
    //
    pub lua_stackdepth: unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int,
    pub lua_getinfo: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        level: c_int,
        what: *const c_char,
        ar: *mut lua_Debug,
    ) -> c_int,
    pub lua_getargument: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        level: c_int,
        n: c_int,
    ) -> c_int,
    pub lua_getlocal: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        level: c_int,
        n: c_int,
    ) -> *const c_char,
    pub lua_setlocal: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        level: c_int,
        n: c_int,
    ) -> *const c_char,
    pub lua_getupvalue: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        funcindex: c_int,
        n: c_int,
    ) -> *const c_char,
    pub lua_setupvalue: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        funcindex: c_int,
        n: c_int,
    ) -> *const c_char,

    pub lua_singlestep: unsafe extern "C-unwind" fn(state: *mut lua_State, enabled: c_int),
    pub lua_breakpoint: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        funcindex: c_int,
        line: c_int,
        enabled: c_int,
    ) -> c_int,

    pub lua_getcoverage: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        funcindex: c_int,
        context: *mut c_void,
        callback: lua_Coverage,
    ),

    pub lua_debugtrace: unsafe extern "C-unwind" fn(state: *mut lua_State) -> *const c_char,

    //
    // Callbacks
    //
    pub lua_callbacks: unsafe extern "C" fn(state: *mut lua_State) -> *mut lua_Callbacks,

    //
    // Customization lib
    //
    pub luau_setfflag: unsafe extern "C" fn(name: *const c_char, value: c_int) -> c_int,
    pub lua_getmetatablepointer: unsafe extern "C" fn(
        state: *mut lua_State,
        idx: c_int,
    ) -> *const c_void,
    pub lua_gcdump: unsafe extern "C" fn(
        state: *mut lua_State,
        file: *mut c_void,
        category_name: Option<unsafe extern "C" fn(state: *mut lua_State, memcat: u8) -> *const c_char>,
    ),

    //
    // luau_try
    //
    // pub luau_try: unsafe extern "C-unwind" fn(
    //     state: *mut lua_State,
    //     func: RustCallback,
    //     data: *mut c_void,
    // ) -> RustCallbackRet,

    //
    // Inline helpers / macros implemented as Rust functions
    //
    pub lua_tonumber: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> lua_Number,
    pub lua_tointeger_: unsafe extern "C-unwind" fn(state: *mut lua_State, idx: c_int) -> c_int,
    pub lua_tounsigned: unsafe extern "C-unwind" fn(state: *mut lua_State, i: c_int) -> lua_Unsigned,
    pub lua_pop: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int),
    pub lua_newtable: unsafe extern "C-unwind" fn(state: *mut lua_State),
    pub lua_newuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, sz: usize) -> *mut c_void,
    pub lua_newuserdata_t: unsafe extern "C-unwind" fn(state: *mut lua_State, data: c_void) -> *mut c_void,
    pub lua_isfunction: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_istable: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_islightuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isnil: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isboolean: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isvector: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isthread: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isbuffer: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isnone: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_isnoneornil: unsafe extern "C-unwind" fn(state: *mut lua_State, n: c_int) -> c_int,
    pub lua_pushliteral: unsafe extern "C-unwind" fn(state: *mut lua_State, s: *const c_char),
    pub lua_pushcfunction: unsafe extern "C-unwind" fn(state: *mut lua_State, f: lua_CFunction),
    pub lua_pushcfunctiond: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        debugname: *const c_char,
    ),
    pub lua_pushcclosure: unsafe extern "C-unwind" fn(state: *mut lua_State, f: lua_CFunction, nup: c_int),
    pub lua_pushcclosurec: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        cont: lua_Continuation,
        nup: c_int,
    ),
    pub lua_pushcclosured: unsafe extern "C-unwind" fn(
        state: *mut lua_State,
        f: lua_CFunction,
        debugname: *const c_char,
        nup: c_int,
    ),
    pub lua_pushlightuserdata: unsafe extern "C-unwind" fn(state: *mut lua_State, p: *mut c_void),
    pub lua_setglobal: unsafe extern "C-unwind" fn(state: *mut lua_State, var: *const c_char),
    pub lua_getglobal: unsafe extern "C-unwind" fn(state: *mut lua_State, var: *const c_char) -> c_int,
    pub lua_tostring: unsafe extern "C-unwind" fn(state: *mut lua_State, i: c_int) -> *const c_char,

    // --- lauxlib additions ---

    pub luaL_register: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        libname: *const c_char,
        l: *const luaL_Reg,
    ),

    pub luaL_getmetafield_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        obj: c_int,
        e: *const c_char,
    ) -> c_int,

    pub luaL_callmeta: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        obj: c_int,
        e: *const c_char,
    ) -> c_int,

    pub luaL_typeerror: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        tname: *const c_char,
    ) -> !,

    pub luaL_argerror: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        extramsg: *const c_char,
    ) -> !,

    pub luaL_checklstring: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        l: *mut usize,
    ) -> *const c_char,

    pub luaL_optlstring: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: *const c_char,
        l: *mut usize,
    ) -> *const c_char,

    pub luaL_checknumber: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> lua_Number,

    pub luaL_optnumber: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: lua_Number,
    ) -> lua_Number,

    pub luaL_checkboolean: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> c_int,

    pub luaL_optboolean: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: c_int,
    ) -> c_int,

    pub luaL_checkinteger_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> c_int,

    pub luaL_optinteger_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: c_int,
    ) -> c_int,

    pub luaL_checkunsigned: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> lua_Unsigned,

    pub luaL_optunsigned: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: lua_Unsigned,
    ) -> lua_Unsigned,

    pub luaL_checkvector: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ) -> *const c_float,

    pub luaL_optvector: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: *const c_float,
    ) -> *const c_float,

    pub luaL_checkstack_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        sz: c_int,
        msg: *const c_char,
    ),

    pub luaL_checktype: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        t: c_int,
    ),

    pub luaL_checkany: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
    ),

    pub luaL_newmetatable_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        tname: *const c_char,
    ) -> c_int,

    pub luaL_checkudata: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        ud: c_int,
        tname: *const c_char,
    ) -> *mut c_void,

    pub luaL_checkbuffer: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        len: *mut usize,
    ) -> *mut c_void,

    pub luaL_where: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        lvl: c_int,
    ),

    pub luaL_error: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        fmt: *const c_char,
        // ...,
    ) -> !,

    pub luaL_checkoption: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        narg: c_int,
        def: *const c_char,
        lst: *const *const c_char,
    ) -> c_int,

    pub luaL_tolstring_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        idx: c_int,
        len: *mut usize,
    ) -> *const c_char,

    pub luaL_newstate: unsafe extern "C-unwind" fn() -> *mut lua_State,

    pub luaL_findtable: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        idx: c_int,
        fname: *const c_char,
        szhint: c_int,
    ) -> *const c_char,

    pub luaL_typename: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        idx: c_int,
    ) -> *const c_char,

    pub luaL_callyieldable: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        nargs: c_int,
        nresults: c_int,
    ) -> c_int,

    pub luaL_sandbox_: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
    ),

    pub luaL_sandboxthread: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
    ),

    // buffer API
    pub luaL_buffinit: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        B: *mut luaL_Strbuf,
    ),

    pub luaL_buffinitsize: unsafe extern "C-unwind" fn(
        L: *mut lua_State,
        B: *mut luaL_Strbuf,
        size: usize,
    ) -> *mut c_char,

    pub luaL_prepbuffsize: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
        size: usize,
    ) -> *mut c_char,

    pub luaL_addlstring: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
        s: *const c_char,
        l: usize,
    ),

    pub luaL_addvalue: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
    ),

    pub luaL_addvalueany: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
        idx: c_int,
    ),

    pub luaL_pushresult: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
    ),

    pub luaL_pushresultsize: unsafe extern "C-unwind" fn(
        B: *mut luaL_Strbuf,
        size: usize,
    ),
}