# html/dom/elements/global-attributes/dir-bdi-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir-bdi-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTML Test: BDI: script adds a bdi element with R text and the direction should be RTL</title>
<meta charset="utf-8">
<meta name="assert" content="The dir global attribute defaults to auto on the bdi element" />
<link rel="author" title="HTML5 bidi test WG" href="mailto:myid.shin@igalia.com" />
<link rel="help" href="https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-bdi-element" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<div id="test1"><bdi>اختبر SomeText</bdi><br></div>
<div id="test2"></div>
<script>
test(() => {
  assert_equals(getComputedStyle(test1.firstChild).direction, "rtl");

  const bdi = document.createElement("bdi");
  var text = document.createTextNode('اختبر SomeText');
  bdi.append(text);
  test2.append(bdi);

  assert_equals(getComputedStyle(test2.firstChild).direction, "rtl");
}, 'BDI test: Directionality');
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
  "source_name": "html/dom/elements/global-attributes/dir-bdi-script.html"
}
```
