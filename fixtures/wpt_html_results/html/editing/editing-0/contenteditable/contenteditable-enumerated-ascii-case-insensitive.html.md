# html/editing/editing-0/contenteditable/contenteditable-enumerated-ascii-case-insensitive.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/contenteditable/contenteditable-enumerated-ascii-case-insensitive.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#attr-contenteditable">
<link rel="help" href="https://html.spec.whatwg.org/#enumerated-attribute">
<meta name="assert" content="@contenteditable values are ASCII case-insensitive">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
function testValue(value, isValid) {
  const valueLower = value.toLowerCase();

  test(() => {
    const el = document.createElement('div');
    if (valueLower !== "inherit") {
      el.setAttribute('contenteditable', value);
    }
    assert_equals(el.contentEditable, isValid ? valueLower : "inherit");
  }, `IDL attribute getter for attribute value "${value}"`);

  test(() => {
    const el = document.createElement('div');
    if (isValid) {
      el.contentEditable = value;
      assert_equals(el.getAttribute('contenteditable'), valueLower === "inherit" ? null : valueLower);
    } else {
      assert_throws_dom("SyntaxError", () => {
        el.contentEditable = value;
      });
    }
  }, `IDL attribute setter for value "${value}"`);
}

const valid = ["true", "false", "inherit", "plaintext-only"]; // "inherit" is treated specially
const invalid = ["foobar", "falſe", "plaıntext-only", "plaİntext-only"];

for (const value of valid) {
  testValue(value, true);
  testValue(value.toUpperCase(), true);
}

for (const value of invalid) {
  testValue(value, false);
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/editing-0/contenteditable/contenteditable-enumerated-ascii-case-insensitive.html"
}
```
