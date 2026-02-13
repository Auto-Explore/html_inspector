# html/dom/elements/global-attributes/classlist-nonstring.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/classlist-nonstring.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>classList: non-string contains</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#classes">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#domtokenlist">
<link rel="help" href="https://webidl.spec.whatwg.org/#es-DOMString">
<link rel="help" href="http://www.ecma-international.org/publications/files/ECMA-ST/ECMA-262.pdf#page=57">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<ul>
<li class=undefined>
<li class=null>
<li class=0>
<li class=NaN>
<li class=Infinity>
<li class=-Infinity>
</ul>
<script>
var items = document.getElementById("test")
                    .getElementsByTagName("li");
var tests = [undefined, null, -0, +0, NaN, +Infinity, -Infinity];
var results = [
  [true,  false, false, false, false, false, false], // "undefined"
  [false, true,  false, false, false, false, false], // "null"
  [false, false, true,  true,  false, false, false], // "0"
  [false, false, false, false, true,  false, false], // "NaN"
  [false, false, false, false, false, true,  false], // "Infinity"
  [false, false, false, false, false, false, true ]  // "-Infinity"
];
</script>
</div>
<script>
test(function() {
  for (var i = 0, il = items.length; i < il; ++i) {
    test(function() {
      for (var j = 0, jl = tests.length; j < jl; ++j) {
        assert_equals(items[i].classList.contains(tests[j]), results[i][j]);
      }
    })
  }
})
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/elements/global-attributes/classlist-nonstring.html"
}
```
