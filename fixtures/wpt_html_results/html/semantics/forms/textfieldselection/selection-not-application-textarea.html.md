# html/semantics/forms/textfieldselection/selection-not-application-textarea.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/textfieldselection/selection-not-application-textarea.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>text field selection (textarea)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#textFieldSelection">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  test(function() {
    var el = document.createElement("textarea");
    assert_equals(el.selectionStart, 0, "initial selectionStart");
    assert_equals(el.selectionEnd, 0, "initial selectionEnd");
    // The initial selection direction must be "none" if the platform supports that
    // direction, or "forward" otherwise.
    assert_in_array(el.selectionDirection, ["none", "forward"]);

    const initialDirection = el.selectionDirection;
    el.selectionDirection = "none";
    assert_equals(el.selectionDirection, initialDirection);

    el.value = "foo";
    el.selectionStart = 1;
    el.selectionEnd = 1;
    el.selectionDirection = "forward";
    assert_equals(el.selectionStart, 1, "updated selectionStart");
    assert_equals(el.selectionEnd, 1, "updated selectionEnd");
    assert_equals(el.selectionDirection, "forward", "updated selectionDirection");

    el.setRangeText("foobar");
    el.setSelectionRange(0, 1);
    assert_equals(el.selectionStart, 0, "final selectionStart");
    assert_equals(el.selectionEnd, 1, "final selectionEnd");
    assert_in_array(el.selectionDirection, ["none", "forward"]);

    const finalDirection = el.selectionDirection;
    el.finalDirection = "forward";
    assert_equals(el.selectionDirection, finalDirection);
  }, "text field selection for the input textarea");
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
  "source_name": "html/semantics/forms/textfieldselection/selection-not-application-textarea.html"
}
```
