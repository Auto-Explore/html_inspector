# html/dom/documents/dom-tree-accessors/document.title-03.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.title-03.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title> document.title  and space normalization  </title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#document.title">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
function test_title(set, expected) {
  test(function() {
      document.title = set;
      assert_equals(document.title, expected);
  }, "document.title after setting to " + format_value(set));
}

test(function() {
  // Single space characters must be normalized. (WHATWG r4353)
  assert_equals(document.title, "document.title and space normalization");
}, "document.title initial value");

test_title("one space", "one space");
test_title("two  spaces", "two spaces");
test_title("one\ttab", "one tab");
test_title("two\t\ttabs", "two tabs");
test_title("one\nnewline", "one newline");
test_title("two\n\nnewlines", "two newlines");
test_title("one\fform feed", "one form feed");
test_title("two\f\fform feeds", "two form feeds");
test_title("one\rcarriage return", "one carriage return");
test_title("two\r\rcarriage returns", "two carriage returns");
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.title-03.html"
}
```
