input-language Rust
decl-version 2.0
var-comparability implicit

ppt ..square(int,\_bool):::ENTER
    ppt-type enter
    variable x
        var-kind variable
        rep-type int
        dec-type int
        flags is_param 
        comparability 1
    variable y
        var-kind variable
        rep-type boolean
        dec-type bool
        flags is_param 
        comparability 2

ppt ..square(int,\_bool):::EXIT0
    ppt-type subexit
    variable x
        var-kind variable
        rep-type int
        dec-type int
        flags is_param 
        comparability 1
    variable y
        var-kind variable
        rep-type boolean
        dec-type bool
        flags is_param 
        comparability 2
    variable return
        var-kind variable
        rep-type int
        dec-type int
        comparability 1
