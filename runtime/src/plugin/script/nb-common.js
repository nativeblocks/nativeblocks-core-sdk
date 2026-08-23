// Nativeblocks script runtime prelude
// Preloaded once per QuickJS context, before any generated script runs.
//
// Contract:
//   - Host natives assumed present: getVariable, updateVariable,
//     __hostNow, __hostDiagnostic
//   - Every function here is TOTAL: it never throws, for any input.
//   - Every function accepts null/undefined/wrong-type and returns a neutral value.
//   - Nothing ever returns undefined; __t is applied at every boundary.
//   - Semantics live here, not in the emitters. Three SDKs emit shape only.
//
// Version must be bumped on any behavior change; it is part of the bytecode cache key.

var __VERSION = 1;

var __MAX_LIST = 100000;
var __MAX_STR = 1048576; // 1 MB

function __diag(msg) {
    if (typeof __hostDiagnostic === "function") __hostDiagnostic(msg);
}

function __capList(a) {
    if (a.length > __MAX_LIST) {
        __diag("list truncated at " + __MAX_LIST);
        return a.slice(0, __MAX_LIST);
    }
    return a;
}

function __capStr(s) {
    if (s.length > __MAX_STR) {
        __diag("string truncated at " + __MAX_STR);
        return s.slice(0, __MAX_STR);
    }
    return s;
}

// Objects built here are prototype-less: a script key of "__proto__" or
// "constructor" is then an ordinary own property, not a pollution vector.
function __newObj() {
    return Object.create(null);
}

function __own(o, k) {
    return Object.prototype.hasOwnProperty.call(o, k);
}

// ---------------------------------------------------------------------------
// Coercion. The only boundary rules in the language.
// ---------------------------------------------------------------------------

function __t(x) {
    return x === undefined ? null : x;
}

/**
 * AnyExpr.asStr, NumExpr.asString - total string coercion.
 *   __str(null) -> ""
 *   __str(2.5) -> "2.5"
 *   __str({a: 1}) -> '{"a":1}'
 */
function __str(x) {
    if (x === null || x === undefined) return "";
    if (typeof x === "string") return __capStr(x);
    if (typeof x === "number") return isFinite(x) ? String(x) : "0";
    if (typeof x === "boolean") return x ? "true" : "false";
    try {
        const s = JSON.stringify(x);
        return s === undefined ? "" : __capStr(s);
    } catch (e) {
        return "";
    }
}

/**
 * AnyExpr.asNum, StrExpr.toNumber - total number coercion.
 *   __num(" 42 ") -> 42
 *   __num("nope") -> 0
 *   __num(true) -> 1
 */
function __num(x) {
    if (typeof x === "number") return isFinite(x) ? x : 0;
    if (typeof x === "boolean") return x ? 1 : 0;
    if (typeof x === "string") {
        const s = x.trim();
        if (s === "") return 0;
        const n = Number(s);
        return isFinite(n) ? n : 0;
    }
    return 0;
}

function __int(x) {
    const n = __num(x);
    return n < 0 ? Math.ceil(n) : Math.floor(n);
}

/**
 * AnyExpr.asBool, StrExpr.toBool - only true, "true", 1 and "1" are true.
 *   __bool("true") -> true
 *   __bool("yes") -> false
 */
function __bool(x) {
    return x === true || x === "true" || x === 1 || x === "1";
}

/**
 * AnyExpr.asList - non-arrays become the empty list.
 *   __list([1, 2]) -> [1, 2]
 *   __list("x") -> []
 */
function __list(x) {
    return Array.isArray(x) ? x : [];
}

/**
 * AnyExpr.asObj - arrays, null and scalars become the empty object.
 *   __obj({a: 1}) -> {a: 1}
 *   __obj([1]) -> {}
 */
function __obj(x) {
    return x !== null && typeof x === "object" && !Array.isArray(x) ? x : __newObj();
}

/**
 * StrExpr.json - parse a JSON string, null when it does not parse.
 *   __json('{"a":1}') -> {a: 1}
 *   __json("oops") -> null
 */
function __json(x) {
    if (typeof x !== "string") return __t(x);
    const s = x.trim();
    if (s === "") return null;
    try {
        return __t(JSON.parse(s));
    } catch (e) {
        return null;
    }
}

/**
 * NbExpr.stringify - JSON text of any value; strings pass through.
 *   __stringify([1, 2]) -> "[1,2]"
 *   __stringify("a") -> "a"
 */
function __stringify(x) {
    if (typeof x === "string") return __capStr(x);
    try {
        const s = JSON.stringify(x);
        return s === undefined ? "" : __capStr(s);
    } catch (e) {
        return "";
    }
}

// Predicates are strictly boolean. JS truthiness is never used.
function __truthy(x) {
    return x === true;
}

// Identity key for value equality: distinguishes 1 from "1" and null from "".
function __key(x) {
    if (x === null || x === undefined) return "z";
    const t = typeof x;
    if (t === "string") return "s" + x;
    if (t === "number") return "n" + (isFinite(x) ? x : 0);
    if (t === "boolean") return "b" + (x ? "1" : "0");
    return "j" + __stringify(x);
}

// ---------------------------------------------------------------------------
// Null-safe access
// ---------------------------------------------------------------------------

/**
 * ObjExpr.field - own-property read; null for a missing key or non-object.
 *   __get({a: 1}, "a") -> 1
 *   __get(null, "a") -> null
 *   __get({}, "__proto__") -> null
 */
function __get(o, k) {
    if (o === null || o === undefined || typeof o !== "object") return null;
    if (!__own(o, k)) return null;
    return __t(o[k]);
}

/**
 * ObjExpr.has - own-property test.
 *   __has({a: 1}, "a") -> true
 *   __has(null, "a") -> false
 */
function __has(o, k) {
    return o !== null && o !== undefined && typeof o === "object" && __own(o, k);
}

/**
 * ListExpr.at, first, last - negative indexes count from the end.
 *   __at([1, 2, 3], 0) -> 1
 *   __at([1, 2, 3], -1) -> 3
 *   __at([1, 2, 3], 9) -> null
 */
function __at(xs, i) {
    const a = __list(xs);
    const n = __int(i);
    const idx = n < 0 ? a.length + n : n;
    if (idx < 0 || idx >= a.length) return null;
    return __t(a[idx]);
}

/**
 * NbExpr.orElse - replaces null/undefined only; 0 and "" are kept.
 *   __or(null, "x") -> "x"
 *   __or(0, "x") -> 0
 */
function __or(x, y) {
    return x === null || x === undefined ? __t(y) : x;
}

// ---------------------------------------------------------------------------
// Numbers
// ---------------------------------------------------------------------------

/**
 * NumExpr.div - division by zero yields 0.
 *   __div(7, 2) -> 3.5
 *   __div(1, 0) -> 0
 */
function __div(a, b) {
    const d = __num(b);
    return d === 0 ? 0 : __num(a) / d;
}

/**
 * NumExpr.mod - modulo by zero yields 0.
 *   __mod(7, 3) -> 1
 *   __mod(1, 0) -> 0
 */
function __mod(a, b) {
    const d = __num(b);
    return d === 0 ? 0 : __num(a) % d;
}

/**
 * NumExpr.coerceIn - reversed bounds are swapped.
 *   __clamp(9, 0, 5) -> 5
 *   __clamp(5, 10, 0) -> 5
 */
function __clamp(x, lo, hi) {
    const v = __num(x);
    let l = __num(lo);
    let h = __num(hi);
    if (l > h) {
        const tmp = l;
        l = h;
        h = tmp;
    }
    return Math.min(Math.max(v, l), h);
}

// ---------------------------------------------------------------------------
// Strings (no regex anywhere)
// ---------------------------------------------------------------------------

/**
 * StrExpr.lowercase.
 *   __lower("AB") -> "ab"
 *   __lower(null) -> ""
 */
function __lower(s) {
    return __str(s).toLowerCase();
}

/**
 * StrExpr.uppercase.
 *   __upper("ab") -> "AB"
 */
function __upper(s) {
    return __str(s).toUpperCase();
}

/**
 * StrExpr.trim.
 *   __trim("  a  ") -> "a"
 */
function __trim(s) {
    return __str(s).trim();
}

/**
 * StrExpr.length.
 *   __len("abc") -> 3
 *   __len(null) -> 0
 */
function __len(s) {
    return __str(s).length;
}

/**
 * StrExpr.isBlank - empty or whitespace only.
 *   __blank("  ") -> true
 *   __blank(null) -> true
 */
function __blank(s) {
    return __str(s).trim().length === 0;
}

/**
 * StrExpr.concat - both sides coerced to string.
 *   __cat("a", 1) -> "a1"
 *   __cat("a", null) -> "a"
 */
function __cat(a, b) {
    return __capStr(__str(a) + __str(b));
}

/**
 * StrExpr.contains - substring test.
 *   __sContains("hello", "ell") -> true
 *   __sContains(null, "a") -> false
 */
function __sContains(s, t) {
    return __str(s).indexOf(__str(t)) >= 0;
}

/**
 * StrExpr.startsWith.
 *   __starts("hello", "he") -> true
 */
function __starts(s, t) {
    return __str(s).indexOf(__str(t)) === 0;
}

/**
 * StrExpr.endsWith - the empty suffix always matches.
 *   __ends("hello", "lo") -> true
 *   __ends("hello", "") -> true
 */
function __ends(s, t) {
    const a = __str(s);
    const b = __str(t);
    if (b.length === 0) return true;
    if (b.length > a.length) return false;
    return a.slice(a.length - b.length) === b;
}

/**
 * StrExpr.split - an empty separator splits into characters.
 *   __split("a,b", ",") -> ["a", "b"]
 *   __split("ab", "") -> ["a", "b"]
 */
function __split(s, t) {
    const sep = __str(t);
    if (sep === "") return __capList(__str(s).split(""));
    return __capList(__str(s).split(sep));
}

/**
 * StrExpr.replace - replaces every occurrence, no regex.
 *   __replace("a-b-c", "-", "+") -> "a+b+c"
 *   __replace("abc", "", "x") -> "abc"
 */
function __replace(s, a, b) {
    const from = __str(a);
    if (from === "") return __str(s);
    return __capStr(__str(s).split(from).join(__str(b)));
}

/**
 * StrExpr.substring - indexes are clamped to the string.
 *   __sub("hello", 1, 3) -> "el"
 *   __sub("hello", 3, 1) -> ""
 *   __sub("hello", 0, 99) -> "hello"
 */
function __sub(s, a, b) {
    const str = __str(s);
    const start = Math.max(0, Math.min(__int(a), str.length));
    const end = Math.max(start, Math.min(__int(b), str.length));
    return str.slice(start, end);
}

// ---------------------------------------------------------------------------
// Collections. Every list op routes through here.
// ---------------------------------------------------------------------------

/**
 * ListExpr.size, count, isEmpty.
 *   __size([1, 2]) -> 2
 *   __size(null) -> 0
 */
function __size(xs) {
    return __list(xs).length;
}

/**
 * ListExpr.map.
 *   __map([1, 2], (x) => __numAdd(x, 1)) -> [2, 3]
 */
function __map(xs, f) {
    const a = __list(xs);
    const out = [];
    for (let i = 0; i < a.length; i++) out.push(__t(f(__t(a[i]))));
    return __capList(out);
}

/**
 * ListExpr.flatMap - non-list results are dropped.
 *   __flatMap([1, 2], (x) => [x, x]) -> [1, 1, 2, 2]
 */
function __flatMap(xs, f) {
    const a = __list(xs);
    const out = [];
    for (let i = 0; i < a.length; i++) {
        const r = __list(f(__t(a[i])));
        for (let j = 0; j < r.length; j++) {
            out.push(__t(r[j]));
            if (out.length > __MAX_LIST) return __capList(out);
        }
    }
    return __capList(out);
}

/**
 * ListExpr.filter - keeps elements whose predicate is exactly true.
 *   __filter([1, 2, 3], (x) => __gt(x, 1)) -> [2, 3]
 */
function __filter(xs, f) {
    const a = __list(xs);
    const out = [];
    for (let i = 0; i < a.length; i++) {
        const x = __t(a[i]);
        if (__truthy(f(x))) out.push(x);
    }
    return out;
}

/**
 * ListExpr.filterNot.
 *   __filterNot([1, 2, 3], (x) => __gt(x, 1)) -> [1]
 */
function __filterNot(xs, f) {
    const a = __list(xs);
    const out = [];
    for (let i = 0; i < a.length; i++) {
        const x = __t(a[i]);
        if (!__truthy(f(x))) out.push(x);
    }
    return out;
}

/**
 * ListExpr.find - null when nothing matches.
 *   __find([1, 2], (x) => __gt(x, 1)) -> 2
 *   __find([1], (x) => __gt(x, 9)) -> null
 */
function __find(xs, f) {
    const a = __list(xs);
    for (let i = 0; i < a.length; i++) {
        const x = __t(a[i]);
        if (__truthy(f(x))) return x;
    }
    return null;
}

/**
 * ListExpr.any.
 *   __any([1, 2], (x) => __gt(x, 1)) -> true
 */
function __any(xs, f) {
    const a = __list(xs);
    for (let i = 0; i < a.length; i++) if (__truthy(f(__t(a[i])))) return true;
    return false;
}

/**
 * ListExpr.all - true for the empty list.
 *   __all([2, 3], (x) => __gt(x, 1)) -> true
 *   __all([], (x) => false) -> true
 */
function __all(xs, f) {
    const a = __list(xs);
    for (let i = 0; i < a.length; i++) if (!__truthy(f(__t(a[i])))) return false;
    return true;
}

/**
 * ListExpr.none.
 *   __none([1], (x) => __gt(x, 9)) -> true
 */
function __none(xs, f) {
    return !__any(xs, f);
}

// Numeric when both keys are numbers, string compare otherwise. Nulls last.
function __cmp(a, b) {
    const an = a === null || a === undefined;
    const bn = b === null || b === undefined;
    if (an && bn) return 0;
    if (an) return 1;
    if (bn) return -1;
    if (typeof a === "number" && typeof b === "number") {
        return a < b ? -1 : a > b ? 1 : 0;
    }
    const x = __str(a);
    const y = __str(b);
    return x < y ? -1 : x > y ? 1 : 0;
}

/**
 * ListExpr.sortBy (dir 1), sortDescBy (dir -1) - stable, nulls last.
 *   __sortBy([3, 1, 2], (x) => x, 1) -> [1, 2, 3]
 *   __sortBy([3, 1, 2], (x) => x, -1) -> [3, 2, 1]
 */
function __sortBy(xs, f, dir) {
    const a = __list(xs).slice();
    const d = __num(dir) < 0 ? -1 : 1;
    // Decorate to call the key function once per element.
    const keyed = [];
    for (let i = 0; i < a.length; i++) keyed.push([__t(f(__t(a[i]))), a[i], i]);
    keyed.sort(function (p, q) {
        const c = __cmp(p[0], q[0]);
        if (c !== 0) return d * c;
        return p[2] - q[2]; // stable
    });
    const out = [];
    for (let j = 0; j < keyed.length; j++) out.push(__t(keyed[j][1]));
    return out;
}

/**
 * ListExpr.groupBy - keys are coerced to strings.
 *   __groupBy([1, 2, 3], (x) => __mod(x, 2)) -> {"1": [1, 3], "0": [2]}
 */
function __groupBy(xs, f) {
    const a = __list(xs);
    const o = __newObj();
    for (let i = 0; i < a.length; i++) {
        const x = __t(a[i]);
        const k = __str(f(x));
        if (!__own(o, k)) o[k] = [];
        o[k].push(x);
    }
    return o;
}

/**
 * ListExpr.associateBy - later elements win on a repeated key.
 *   __associateBy([{id: "a"}], (x) => __get(x, "id")) -> {"a": {id: "a"}}
 */
function __associateBy(xs, f) {
    const a = __list(xs);
    const o = __newObj();
    for (let i = 0; i < a.length; i++) {
        const x = __t(a[i]);
        o[__str(f(x))] = x;
    }
    return o;
}

/**
 * ListExpr.sumOf - non-numeric selections count as 0.
 *   __sumOf([{v: 2}, {v: 3}], (x) => __get(x, "v")) -> 5
 *   __sumOf([], (x) => x) -> 0
 */
function __sumOf(xs, f) {
    const a = __list(xs);
    let s = 0;
    for (let i = 0; i < a.length; i++) s += __num(f(__t(a[i])));
    return isFinite(s) ? s : 0;
}

/**
 * ListExpr.fold - left fold from an initial value.
 *   __fold([1, 2, 3], 0, (acc, x) => __numAdd(acc, x)) -> 6
 */
function __fold(xs, init, f) {
    const a = __list(xs);
    let acc = __t(init);
    for (let i = 0; i < a.length; i++) acc = __t(f(acc, __t(a[i])));
    return acc;
}

/**
 * ListExpr.take - negative counts take nothing.
 *   __take([1, 2, 3], 2) -> [1, 2]
 *   __take([1, 2, 3], -1) -> []
 */
function __take(xs, n) {
    return __list(xs).slice(0, Math.max(0, __int(n)));
}

/**
 * ListExpr.drop.
 *   __drop([1, 2, 3], 2) -> [3]
 *   __drop([1, 2, 3], 9) -> []
 */
function __drop(xs, n) {
    return __list(xs).slice(Math.max(0, __int(n)));
}

/**
 * ListExpr.slice - indexes are clamped to the list.
 *   __slice([1, 2, 3], 1, 3) -> [2, 3]
 *   __slice([1, 2, 3], 2, 0) -> []
 */
function __slice(xs, a, b) {
    const arr = __list(xs);
    const start = Math.max(0, Math.min(__int(a), arr.length));
    const end = Math.max(start, Math.min(__int(b), arr.length));
    return arr.slice(start, end);
}

/**
 * ListExpr.distinct - identity is type aware, so 1 and "1" both survive.
 *   __distinct([1, 1, 2]) -> [1, 2]
 *   __distinct([1, "1"]) -> [1, "1"]
 */
function __distinct(xs) {
    const a = __list(xs);
    const seen = __newObj();
    const out = [];
    for (let i = 0; i < a.length; i++) {
        const k = __key(a[i]);
        if (!__own(seen, k)) {
            seen[k] = 1;
            out.push(__t(a[i]));
        }
    }
    return out;
}

/**
 * ListExpr.reversed - does not mutate the input.
 *   __rev([1, 2, 3]) -> [3, 2, 1]
 */
function __rev(xs) {
    return __list(xs).slice().reverse();
}

/**
 * ListExpr.flatten - one level; non-list elements are dropped.
 *   __flatten([[1], [2, 3]]) -> [1, 2, 3]
 */
function __flatten(xs) {
    const a = __list(xs);
    const out = [];
    for (let i = 0; i < a.length; i++) {
        const inner = __list(a[i]);
        for (let j = 0; j < inner.length; j++) {
            out.push(__t(inner[j]));
            if (out.length > __MAX_LIST) return __capList(out);
        }
    }
    return out;
}

/**
 * ListExpr.add - appends one element.
 *   __add([1], 2) -> [1, 2]
 */
function __add(xs, x) {
    return __capList(__list(xs).concat([__t(x)]));
}

/**
 * ListExpr.addAll - appends every element of another list.
 *   __addAll([1], [2, 3]) -> [1, 2, 3]
 */
function __addAll(xs, ys) {
    return __capList(__list(xs).concat(__list(ys)));
}

/**
 * ListExpr.insertAt - inserts before the index, shifting the rest right.
 * The index is clamped, so a negative index prepends and a large one appends.
 *   __insertAt([1, 3], 1, 2) -> [1, 2, 3]
 *   __insertAt([1, 2], 9, 3) -> [1, 2, 3]
 */
function __insertAt(xs, i, x) {
    const a = __list(xs).slice();
    const at = Math.max(0, Math.min(__int(i), a.length));
    a.splice(at, 0, __t(x));
    return __capList(a);
}

/**
 * ListExpr.setAt - replaces the element at the index; out of range is a no-op.
 *   __setAt([1, 9, 3], 1, 2) -> [1, 2, 3]
 *   __setAt([1], 5, 2) -> [1]
 */
function __setAt(xs, i, x) {
    const a = __list(xs).slice();
    const at = __int(i);
    if (at < 0 || at >= a.length) return a;
    a[at] = __t(x);
    return a;
}

/**
 * ListExpr.remove - removes the first equal element, type aware.
 *   __remove([1, 2, 1], 1) -> [2, 1]
 *   __remove([1], 9) -> [1]
 */
function __remove(xs, x) {
    const a = __list(xs).slice();
    const at = __indexOf(a, x);
    if (at < 0) return a;
    a.splice(at, 1);
    return a;
}

/**
 * ListExpr.removeAll - removes every element that appears in the other list.
 *   __removeAll([1, 2, 1, 3], [1, 3]) -> [2]
 */
function __removeAll(xs, ys) {
    const drop = __newObj();
    const b = __list(ys);
    for (let i = 0; i < b.length; i++) drop[__key(b[i])] = 1;
    const a = __list(xs);
    const out = [];
    for (let i = 0; i < a.length; i++) {
        if (!__own(drop, __key(a[i]))) out.push(__t(a[i]));
    }
    return out;
}

/**
 * ListExpr.removeAt - removes the element at the index; out of range is a no-op.
 *   __removeAt([1, 2, 3], 1) -> [1, 3]
 *   __removeAt([1], 5) -> [1]
 */
function __removeAt(xs, i) {
    const a = __list(xs).slice();
    const at = __int(i);
    if (at < 0 || at >= a.length) return a;
    a.splice(at, 1);
    return a;
}

/**
 * ListExpr.indexOf - position of the first equal element, -1 when absent.
 *   __indexOf(["a", "b"], "b") -> 1
 *   __indexOf([1], "1") -> -1
 */
function __indexOf(xs, x) {
    const k = __key(x);
    const a = __list(xs);
    for (let i = 0; i < a.length; i++) if (__key(a[i]) === k) return i;
    return -1;
}

/**
 * ListExpr.zip - stops at the shorter list.
 *   __zip([1, 2], ["a"]) -> [[1, "a"]]
 */
function __zip(xs, ys) {
    const a = __list(xs);
    const b = __list(ys);
    const n = Math.min(a.length, b.length);
    const out = [];
    for (let i = 0; i < n; i++) out.push([__t(a[i]), __t(b[i])]);
    return out;
}

/**
 * ListExpr.chunked - size is at least 1.
 *   __chunked([1, 2, 3], 2) -> [[1, 2], [3]]
 */
function __chunked(xs, n) {
    const a = __list(xs);
    const s = Math.max(1, __int(n));
    const out = [];
    for (let i = 0; i < a.length; i += s) out.push(a.slice(i, i + s));
    return out;
}

/**
 * ListExpr.contains - type aware, so null does not match "".
 *   __contains([1, 2], 2) -> true
 *   __contains([null], "") -> false
 */
function __contains(xs, x) {
    const k = __key(x);
    const a = __list(xs);
    for (let i = 0; i < a.length; i++) if (__key(a[i]) === k) return true;
    return false;
}

/**
 * ListExpr.joinToString - elements coerced to string.
 *   __join([1, null, "a"], ",") -> "1,,a"
 */
function __join(xs, sep) {
    const a = __list(xs);
    const s = __str(sep);
    const parts = [];
    for (let i = 0; i < a.length; i++) parts.push(__str(a[i]));
    return __capStr(parts.join(s));
}

// ---------------------------------------------------------------------------
// Objects
// ---------------------------------------------------------------------------

/**
 * ObjExpr.keys.
 *   __keys({a: 1, b: 2}) -> ["a", "b"]
 *   __keys(null) -> []
 */
function __keys(o) {
    return Object.keys(__obj(o));
}

/**
 * ObjExpr.values.
 *   __values({a: 1}) -> [1]
 */
function __values(o) {
    const t = __obj(o);
    const ks = Object.keys(t);
    const out = [];
    for (let i = 0; i < ks.length; i++) out.push(__t(t[ks[i]]));
    return out;
}

/**
 * ObjExpr.entries - each entry is a [key, value] pair.
 *   __entries({a: 1}) -> [["a", 1]]
 */
function __entries(o) {
    const t = __obj(o);
    const ks = Object.keys(t);
    const out = [];
    for (let i = 0; i < ks.length; i++) out.push([ks[i], __t(t[ks[i]])]);
    return out;
}

/**
 * ObjExpr.put - sets one key, replacing any existing value.
 *   __put({a: 1}, "b", 2) -> {a: 1, b: 2}
 *   __put({a: 1}, "a", 2) -> {a: 2}
 */
function __put(o, k, v) {
    const out = __putAll(o, null);
    out[__str(k)] = __t(v);
    return out;
}

/**
 * ObjExpr.putAll - the right side wins on a shared key.
 *   __putAll({a: 1}, {a: 2, b: 3}) -> {a: 2, b: 3}
 */
function __putAll(a, b) {
    const out = __newObj();
    const x = __obj(a);
    const y = __obj(b);
    const kx = Object.keys(x);
    for (let i = 0; i < kx.length; i++) out[kx[i]] = __t(x[kx[i]]);
    const ky = Object.keys(y);
    for (let j = 0; j < ky.length; j++) out[ky[j]] = __t(y[ky[j]]);
    return out;
}

/**
 * ObjExpr.remove - drops one key.
 *   __removeKey({a: 1, b: 2}, "a") -> {b: 2}
 */
function __removeKey(o, k) {
    const t = __obj(o);
    const out = __newObj();
    const ks = Object.keys(t);
    for (let i = 0; i < ks.length; i++) if (ks[i] !== k) out[ks[i]] = __t(t[ks[i]]);
    return out;
}

/**
 * ObjExpr.clearAll - a fresh empty object. Emitted as a call because a literal
 * {} would collide with the {placeholder} syntax in the spec's emit templates.
 *   __emptyObj() -> {}
 */
function __emptyObj() {
    return __newObj();
}

/**
 * ObjExpr.size, isEmpty - number of own keys.
 *   __objSize({a: 1, b: 2}) -> 2
 *   __objSize(null) -> 0
 */
function __objSize(o) {
    return Object.keys(__obj(o)).length;
}

/**
 * ObjExpr.mapKeys - every key through the body, collected as a list of strings.
 *   __mapKeys({a: 1, b: 2}, (k) => __upper(k)) -> ["A", "B"]
 */
function __mapKeys(o, f) {
    const t = __obj(o);
    const ks = Object.keys(t);
    const out = [];
    for (let i = 0; i < ks.length; i++) out.push(__str(f(ks[i])));
    return __capList(out);
}

/**
 * ObjExpr.mapValues - every value through the body, collected as a list.
 *   __mapValues({a: 1, b: 2}, (v) => __numAdd(v, 1)) -> [2, 3]
 */
function __mapValues(o, f) {
    const t = __obj(o);
    const ks = Object.keys(t);
    const out = [];
    for (let i = 0; i < ks.length; i++) out.push(__t(f(__t(t[ks[i]]))));
    return __capList(out);
}

// ---------------------------------------------------------------------------
// Comparison and logic
//
// Not emitted by spec v1 (which still inlines ===, >, &&, +). Shipped now so
// the emitters can move over without waiting for a client release: raw JS
// operators make null >= 0 true and let (a && b) return a non-boolean.
// ---------------------------------------------------------------------------

/**
 * Type-aware equality. Not emitted by spec v1.
 *   __eq(1, 1) -> true
 *   __eq(1, "1") -> false
 *   __eq(null, null) -> true
 */
function __eq(a, b) {
    return __key(a) === __key(b);
}

/**
 * Type-aware inequality. Not emitted by spec v1.
 *   __ne(1, "1") -> true
 */
function __ne(a, b) {
    return __key(a) !== __key(b);
}

/**
 * Null test, undefined included. Not emitted by spec v1.
 *   __isNull(null) -> true
 *   __isNull(0) -> false
 */
function __isNull(x) {
    return x === null || x === undefined;
}

/**
 * Greater than; false when either side is null. Not emitted by spec v1.
 *   __gt(2, 1) -> true
 *   __gt(null, 0) -> false
 */
function __gt(a, b) {
    return __isNull(a) || __isNull(b) ? false : __num(a) > __num(b);
}

/**
 * Greater or equal; false when either side is null (raw >= says true).
 *   __gte(1, 1) -> true
 *   __gte(null, 0) -> false
 */
function __gte(a, b) {
    return __isNull(a) || __isNull(b) ? false : __num(a) >= __num(b);
}

/**
 * Less than; false when either side is null. Not emitted by spec v1.
 *   __lt(1, 2) -> true
 */
function __lt(a, b) {
    return __isNull(a) || __isNull(b) ? false : __num(a) < __num(b);
}

/**
 * Less or equal; false when either side is null. Not emitted by spec v1.
 *   __lte(1, 1) -> true
 */
function __lte(a, b) {
    return __isNull(a) || __isNull(b) ? false : __num(a) <= __num(b);
}

/**
 * Boolean and; always a boolean (raw && can return null).
 *   __logAnd(true, true) -> true
 *   __logAnd(true, null) -> false
 */
function __logAnd(a, b) {
    return __truthy(a) && __truthy(b);
}

/**
 * Boolean or; always a boolean.
 *   __logOr(false, true) -> true
 */
function __logOr(a, b) {
    return __truthy(a) || __truthy(b);
}

/**
 * Boolean not; anything that is not true negates to true.
 *   __logNot(null) -> true
 */
function __logNot(a) {
    return !__truthy(a);
}

/**
 * Addition on coerced numbers.
 *   __numAdd(1, 2) -> 3
 *   __numAdd(null, 2) -> 2
 */
function __numAdd(a, b) {
    const n = __num(a) + __num(b);
    return isFinite(n) ? n : 0;
}

/**
 * Subtraction on coerced numbers.
 *   __numSub(3, 1) -> 2
 */
function __numSub(a, b) {
    const n = __num(a) - __num(b);
    return isFinite(n) ? n : 0;
}

/**
 * Multiplication on coerced numbers.
 *   __numMul(3, 2) -> 6
 */
function __numMul(a, b) {
    const n = __num(a) * __num(b);
    return isFinite(n) ? n : 0;
}

/**
 * Absolute value.
 *   __abs(-2) -> 2
 *   __abs(null) -> 0
 */
function __abs(x) {
    return Math.abs(__num(x));
}

/**
 * Round half up.
 *   __round(2.5) -> 3
 */
function __round(x) {
    return Math.round(__num(x));
}

/**
 * Round down.
 *   __floor(2.9) -> 2
 */
function __floor(x) {
    return Math.floor(__num(x));
}

/**
 * Round up.
 *   __ceil(2.1) -> 3
 */
function __ceil(x) {
    return Math.ceil(__num(x));
}

/**
 * Truncate toward zero.
 *   __trunc(-2.9) -> -2
 */
function __trunc(x) {
    return __int(x);
}

/**
 * Smaller of two coerced numbers.
 *   __min(1, 2) -> 1
 */
function __min(a, b) {
    return Math.min(__num(a), __num(b));
}

/**
 * Larger of two coerced numbers.
 *   __max(1, 2) -> 2
 */
function __max(a, b) {
    return Math.max(__num(a), __num(b));
}

/**
 * The match construct. Cases are [condition, value] thunk pairs; the first
 * condition that is exactly true wins, so only the taken branch is evaluated.
 *   __match([[() => false, () => "a"], [() => true, () => "b"]], () => "c") -> "b"
 *   __match([], () => "c") -> "c"
 */
function __match(cases, fallback) {
    const a = __list(cases);
    for (let i = 0; i < a.length; i++) {
        if (__truthy(a[i][0]())) return __t(a[i][1]());
    }
    return __t(fallback());
}

// ---------------------------------------------------------------------------
// Host bridge
// ---------------------------------------------------------------------------

// Host-injected clock. Keeps scripts deterministic in golden tests and avoids
// exposing Date to script authors.
/**
 * Host clock in milliseconds; 0 when the host injects no clock.
 *   __now() -> 1755600000000
 */
function __now() {
    return typeof __hostNow === "function" ? __num(__hostNow()) : 0;
}

/**
 * NbExpr.delay - pause for a number of milliseconds, then yield the receiver.
 * Zero, negative and non-numeric are no-ops, as is a host with no clock.
 *   __delay(50) -> null
 */
function __delay(ms) {
    if (typeof __hostDelay === "function") __hostDelay(__num(ms));
    return null;
}
