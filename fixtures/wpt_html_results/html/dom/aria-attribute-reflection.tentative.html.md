# html/dom/aria-attribute-reflection.tentative.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/aria-attribute-reflection.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>Element Reflection for ARIA properties</title>
<link rel=help href="https://wicg.github.io/aom/spec/aria-reflection.html">
<link rel="author" title="Meredith Lane" href="meredithl@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
function testNullable(element, jsAttr, contentAttr) {
    var originalValue = element[jsAttr];
    assert_false(originalValue === null);
    element[jsAttr] = null;
    assert_equals(element[jsAttr], null);
    assert_false(element.hasAttribute(contentAttr));
    // Setting to undefined results in same state as setting to null.
    element[jsAttr] = originalValue;
    element[jsAttr] = undefined;
    assert_equals(element[jsAttr], null);
    assert_false(element.hasAttribute(contentAttr));
}
</script>

<!-- tentative -->
<div id="colindextext" aria-colindextext="x"></div>
<script>
test(function(t) {
    var element = document.getElementById("colindextext");
    assert_equals(element.ariaColIndexText, "x");
    element.ariaColIndexText = "y";
    assert_equals(element.getAttribute("aria-colindextext"), "y");
    testNullable(element, "ariaColIndexText", "aria-colindextext");
}, "aria-colindextext attribute reflects.");
</script>

<!-- tentative -->
<div id="description" aria-description="cold as ice"></div>
<script>
test(function(t) {
    var element = document.getElementById("description");
    assert_equals(element.ariaDescription, "cold as ice");
    element.ariaDescription = "hot as fire";
    assert_equals(element.getAttribute("aria-description"), "hot as fire");
    testNullable(element, "ariaDescription", "aria-description");
}, "aria-description attribute reflects.");
</script>

<!-- tentative -->
<div id="rowindextext" aria-rowindextext="x"></div>
<script>
    test(function(t) {
        var element = document.getElementById("rowindextext");
        assert_equals(element.ariaRowIndexText, "x");
        element.ariaRowIndexText = "y";
        assert_equals(element.getAttribute("aria-rowindextext"), "y");
        testNullable(element, "ariaRowIndexText", "aria-rowindextext");
    }, "aria-rowindextext attribute reflects.");
</script>

</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 2238,
        "byte_start": 2231,
        "col": 1,
        "line": 60
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/dom/aria-attribute-reflection.tentative.html"
}
```
