{
    let closefile =     fn [Param { name: Some("filename"), pattern: RawToken(Ident("filename")) }] =>     {
        let filehandle =         FnCall("gethandle", [Ident("filename")])    }
    FnCall("print", [FnCall("closefile", ["\"hello.exas\""])])}
