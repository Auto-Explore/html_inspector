# html/dom/documents/dom-tree-accessors/Element.getElementsByClassName-null-undef.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/Element.getElementsByClassName-null-undef.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>getElementsByClassName and null/undefined</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-getelementsbyclassname">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<p id="p1"></p>
<p class="undefined" id="p2"></p>
<p class="null" id="p3"></p>
<p class="undefined null" id="p4"></p>
</div>
<script>
test(function() {
  var wrapper = document.getElementById("test");
  assert_equals(wrapper.getElementsByClassName(undefined).length, 2);
  assert_equals(wrapper.getElementsByClassName(undefined)[0],
    document.getElementById("p2"));
  assert_equals(wrapper.getElementsByClassName(undefined)[1],
    document.getElementById("p4"));
/*
  assert_equals(wrapper.getElementsByClassName(null).length, 2);
  assert_equals(wrapper.getElementsByClassName(null)[0],
    document.getElementById("p3"));
  assert_equals(wrapper.getElementsByClassName(null)[1],
    document.getElementById("p4"));
*/
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
  "source_name": "html/dom/documents/dom-tree-accessors/Element.getElementsByClassName-null-undef.html"
}
```
