# html/dom/elements/global-attributes/dir-auto-div-append-child.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir-auto-div-append-child.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTML Test: input with dir=auto, then append a child</title>
<meta charset="utf-8">
<meta name="assert" content="The dir global attribute set to auto applies when a child is appended" />
<link rel="author" title="HTML5 bidi test WG" href="mailto:japhet@chromium.org" />
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-dir-attribute" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<div id="div" dir="auto"></div>
<script>
test(() => {
  assert_equals(getComputedStyle(div).direction, "ltr");
  div.appendChild(document.createTextNode('اختبر SomeText'));
  assert_equals(getComputedStyle(div).direction, "rtl");
}, 'dir auto: updates on appendChild');
</script>
</body>
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
  "source_name": "html/dom/elements/global-attributes/dir-auto-div-append-child.html"
}
```
