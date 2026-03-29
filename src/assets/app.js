(function(){

"use strict";

/* ===============================
   SAUVEGARDE ACTIVATE ORIGINAL
================================ */

const _activateOriginal = window.activate;

/* ===============================
   SECRET INTERNE (fragmenté)
================================ */

function _s1(){ return [75,57]; }
function _s2(){ return [70,51]; }
function _s3(){ return [80,49]; }
function _s4(){ return [88,55]; }

function _secret(){

    return String.fromCharCode(
        ..._s1(),
        ..._s2(),
        ..._s3(),
        ..._s4()
    );

}

/* ===============================
   EMPREINTE APPAREIL
================================ */

function _device(){

    return navigator.userAgent +
           screen.width +
           screen.height +
           navigator.language +
           Intl.DateTimeFormat().resolvedOptions().timeZone;

}

/* ===============================
   HASH SHA256
================================ */

async function _hash(code,date){

    const data =
        code +
        date +
        _device() +
        _secret();

    const buffer = new TextEncoder().encode(data);

    const digest = await crypto.subtle.digest("SHA-256",buffer);

    return Array.from(new Uint8Array(digest))
        .map(b=>b.toString(16).padStart(2,"0"))
        .join("");

}

/* ===============================
   VALIDATION LICENCE
================================ */

async function _verify(){

    const code = localStorage.getItem("validCode");
    const date = localStorage.getItem("activationDate");
    const sig  = localStorage.getItem("activationSig");

    if(!code || !date || !sig)
        return false;

    const expected = await _hash(code,date);

    return expected === sig;

}

/* ===============================
   WRAPPER ACTIVATE
================================ */

window.activate = function(){

    const result = _activateOriginal();

    try{

        _verify().then(valid=>{

            if(!valid){
                result.isValid = false;
            }

        });

    }catch(e){}

    return result;

};

/* ===============================
   PROTECTION ACTIVATE
================================ */

Object.defineProperty(window,"activate",{
    configurable:false,
    writable:false
});

/* ===============================
   ANTI TAMPER
================================ */

const _src = window.activate.toString();

setInterval(()=>{

    try{

        if(window.activate.toString() !== _src){

            location.reload();

        }

    }catch(e){}

},3000);

})();