# html/semantics/forms/the-form-element/form-elements-interfaces-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-elements-interfaces-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>form.elements: interfaces</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-form-elements">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#htmlformcontrolscollection">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var form = document.createElement("form");
  ["HTMLFormControlsCollection", "HTMLCollection"].forEach(function(i) {
    test(function() {
      assert_true(i in window, "Interface should exist")
      assert_true(form.elements instanceof window[i],
                  "elements should implement the interface")
    }, "Testing interface " + i)
  })
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
  "source_name": "html/semantics/forms/the-form-element/form-elements-interfaces-01.html"
}
```
