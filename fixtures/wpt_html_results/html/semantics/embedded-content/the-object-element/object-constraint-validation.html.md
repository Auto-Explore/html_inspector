# html/semantics/embedded-content/the-object-element/object-constraint-validation.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/object-constraint-validation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>The object element's constraint validation.</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-object-element">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-validitystate-customerror">
<link rel="author" title="Peng Zhou" href="mailto:zhoupeng.1996@bytedance.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<object id="element"></object>
<script>
function check() {
  assert_false(element.willValidate);
  assert_true(element.checkValidity());
  assert_true(element.reportValidity());
}

test(() => {
  check();
  element.setCustomValidity("custom error");
  check();
}, "the object element's constraint validation is correct")
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 461,
        "byte_start": 440,
        "col": 1,
        "line": 8
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
  "source_name": "html/semantics/embedded-content/the-object-element/object-constraint-validation.html"
}
```
