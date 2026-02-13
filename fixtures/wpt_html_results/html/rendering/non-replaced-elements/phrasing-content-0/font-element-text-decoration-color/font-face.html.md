# html/rendering/non-replaced-elements/phrasing-content-0/font-element-text-decoration-color/font-face.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/phrasing-content-0/font-element-text-decoration-color/font-face.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>font face</title>
<link rel="help" href="https://html.spec.whatwg.org/#the-font-element-text-decoration-color-quirk">
<link rel="author" title="Intel" href="http://www.intel.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>

const types = ["serif", "sans-serif", "cursive", "fantasy", "monospace"];
for (let type of types) {
  test(() => {
    let elem = document.createElement("font");
    elem.setAttribute("face", type);
    document.body.appendChild(elem);
    let exp_type = window.getComputedStyle(elem, null).getPropertyValue("font-family");
    assert_equals(exp_type, type);
  }, `font face attribute ${type} is correct`);
}

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
  "source_name": "html/rendering/non-replaced-elements/phrasing-content-0/font-element-text-decoration-color/font-face.html"
}
```
