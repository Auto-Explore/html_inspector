# html/browsers/the-window-object/named-access-on-the-window-object/window-null-names.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/window-null-names.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Named access with null characters</title>
<link rel="author" title="Ms2ger" href="ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#window">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-window-nameditem">
<link rel="help" href="https://webidl.spec.whatwg.org/#named-properties-object">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
test(function() {
  var iframe = document.createElement("iframe")
  iframe.name = "a\0b"
  document.body.appendChild(iframe)
  assert_equals(window["a\0b"], iframe.contentWindow)
  assert_equals(window["ab"], undefined)
  assert_equals(window["a"], undefined)
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
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/window-null-names.html"
}
```
