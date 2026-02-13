# html/semantics/forms/constraints/reportValidity-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/constraints/reportValidity-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>

<head>
<script>
Object.prototype.__defineGetter__('then', prom);
var prom_count = 0;
function prom() {
prom_count++;
if (prom_count > 2) return;
var v14 = x37.animate({},100);
v14.reverse();
v14.ready;
v14.currentTime = 0;
x57.reportValidity();
}
function f0() {
var v38 = x37.animate({},300);
v38.ready;
x57.prepend(x78);
}
function f1() {
var x57 = document.getElementById("x57");
x57.disabled = false;
}
</script>
</head>

<body>
<fieldset id="x37">
<canvas onfocusin="f0()" >
<input id="x78" autofocus=""  onfocusout="f1()" >
</canvas>
<select id="x57" disabled=""  required=""></select>
</body>

</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.select.required.must_have_option",
      "message": "A “select” element with a “required” attribute, and without a “multiple” attribute, and without a “size” attribute whose value is greater than “1”, must have a child “option” element.",
      "severity": "Warning",
      "span": {
        "byte_end": 615,
        "byte_start": 606,
        "col": 43,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/constraints/reportValidity-crash.html"
}
```
