# html/semantics/scripting-1/the-script-element/execution-timing/146-href.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/execution-timing/146-href.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html><head>
  <title>scheduler: SVG script adding src attribute </title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script src="testlib/testlib.js"></script>
</head>
<div id="log"></div>
<script>var t = async_test();</script>
<svg>
<script></script>
</svg>
<script>
t.step(function() {
  var s = document.querySelector("svg > script");
  s.setAttribute("href", "scripts/include-1.js");
});
onload = t.step_func(function() {
  assert_array_equals(eventOrder, ["external script #1"]);
  t.done();
});
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
  "source_name": "html/semantics/scripting-1/the-script-element/execution-timing/146-href.html"
}
```
